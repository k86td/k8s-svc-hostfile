use configuration::SimpleConfiguration;
use k8s_openapi::{
    api::core::v1::{Namespace, Pod, Service},
    Metadata,
};
use kube::{api::ListParams, Api, Client, Resource, ResourceExt};
use log::{debug, info};
use std::{collections::HashMap, env, rc::Rc, sync::Arc, time::Duration};
use tokio::time;

use crate::configuration::{EnvConfigManager, SimpleConfigurationBuilder};

mod configuration;
mod utils;

#[tokio::main]
async fn main() {
    // 1. Get app config
    // 2. Get k8s config (like using Kubeconfig, or token/key, etc.)

    env_logger::init();

    let config = EnvConfigManager::new().build();

    info!(
        "Got config, sleep time: {} sec, namespace selector: {:#?}, service selector: {:#?}",
        config.refresh_duration.as_secs(),
        config.namespaces_selector,
        config.services_selector
    );

    // runtime loop
    loop {
        let client = Client::try_default().await.unwrap();

        let namespace_resource: Api<Namespace> = Api::all(client.clone());
        for namespace in namespace_resource
            .list(&config.namespaces_selector)
            .await
            .unwrap()
        {
            debug!("Found NS: {:#?}", namespace.name_any());

            let mut ns_tld: Option<&str> = None;
            let mut ns_domain: Option<&str> = None;

            if let Some(annotations) = &namespace.metadata().annotations {
                if let Some(tld) = annotations.get("fqdn.tld") {
                    debug!("\t+TLD <{}>", tld);
                    ns_tld = Some(tld);
                }
                if let Some(domain) = annotations.get("fqdn.domain") {
                    debug!("\t+domain <{}>", domain);
                    ns_domain = Some(domain);
                }
            }

            let services_resource: Api<Service> =
                Api::namespaced(client.clone(), &namespace.name_unchecked());

            for service in services_resource.list(&Default::default()).await.unwrap() {
                let mut svc_tld: Option<&str> = None;
                let mut svc_domain: Option<&str> = None;

                debug!("Found Service: {}", service.name_unchecked());
                if let Some(annotations) = &service.metadata().annotations {
                    if let Some(tld) = annotations.get("fqdn.tld") {
                        debug!("\t+TLD <{}>", tld);
                        svc_tld = Some(tld);
                    }
                    if let Some(domain) = annotations.get("fqdn.domain") {
                        debug!("\t+domain <{}>", domain);
                        svc_domain = Some(domain);
                    }
                }

                let full_domain = match (svc_domain.or(ns_domain), svc_tld.or(ns_tld)) {
                    (Some(domain), Some(tld)) => Some(format!("{}.{}", domain, tld)),
                    _ => None,
                };

                if let Some(domain) = full_domain {
                    info!(
                        "Found fqdn <{}> for svc <{}>",
                        domain,
                        service.name_unchecked()
                    );
                }
            }
        }

        time::sleep(config.refresh_duration).await;
    }
}
