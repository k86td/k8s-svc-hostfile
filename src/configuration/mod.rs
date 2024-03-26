use std::{ops::Deref, time::Duration};

use k8s_openapi::api::core::v1::Namespace;
use kube::api::ListParams;

pub struct SimpleConfiguration {
    pub refresh_duration: Duration,
    pub namespaces_selector: ListParams, // not sure if this should be list of Strings
    pub services_selector: ListParams,
}

impl Default for SimpleConfiguration {
    fn default() -> Self {
        Self {
            refresh_duration: Duration::from_secs(60),
            namespaces_selector: Default::default(),
            services_selector: Default::default(),
        }
    }
}

pub trait SimpleConfigurationBuilder {
    fn refresh_duration(&mut self, duration: &Duration) -> &mut Self;
    fn namespaces_selector(&mut self, selector: &ListParams) -> &mut Self;
    fn services_selector(&mut self, selector: &ListParams) -> &mut Self;

    fn build(&mut self) -> SimpleConfiguration;
}

pub struct EnvConfigManager {
    refresh_duration: Option<Duration>,
    namespaces_selector: Option<ListParams>, // not sure if this should be list of Strings
    services_selector: Option<ListParams>,
}

impl SimpleConfigurationBuilder for EnvConfigManager {
    fn refresh_duration(&mut self, duration: &Duration) -> &mut Self {
        self.refresh_duration = Some(*duration);
        self
    }

    fn namespaces_selector(&mut self, selector: &ListParams) -> &mut Self {
        self.namespaces_selector = Some(selector.clone());
        self
    }

    fn services_selector(&mut self, selector: &ListParams) -> &mut Self {
        self.services_selector = Some(selector.clone());
        self
    }

    fn build(&mut self) -> SimpleConfiguration {
        SimpleConfiguration {
            refresh_duration: self.refresh_duration.unwrap_or(Duration::from_secs(60)),
            namespaces_selector: self.namespaces_selector.clone().unwrap_or_default(),
            services_selector: self.services_selector.clone().unwrap_or_default(),
        }
    }
}

impl EnvConfigManager {
    pub fn new() -> EnvConfigManager {
        EnvConfigManager {
            refresh_duration: None,
            namespaces_selector: None,
            services_selector: None,
        }
    }
}
