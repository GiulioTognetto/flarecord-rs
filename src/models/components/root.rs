use twilight_model::channel::message::Component as TwilightComponent;

use crate::{
    models::components::{
        id::{assign_ids, HierarchicalId},
        layout::LayoutComponent,
    },
    traits::component::IntoTwilight,
};

pub struct RootComponent {
    pub(crate) component_id: Option<String>,
    pub(crate) children: Vec<LayoutComponent>,
}

impl RootComponent {
    pub fn new() -> Self {
        Self {
            component_id: None,
            children: Vec::new(),
        }
    }

    pub(crate) fn set_component_id(&mut self, component_id: String) {
        self.component_id = Some(component_id);
    }

    pub(crate) fn require_components_v2(&self) -> bool {
        self.children
            .iter()
            .any(LayoutComponent::require_components_v2)
    }

    pub(crate) fn count(&self) -> usize {
        self.children.iter().map(LayoutComponent::count).sum()
    }

    pub(crate) fn assign_ids(&mut self) {
        let mut id = HierarchicalId::new();

        if let Some(component_id) = &self.component_id {
            id.set_prefix(component_id.clone());
        }

        for child in &mut self.children {
            assign_ids(child, &mut id);
            id.next_root();
        }
    }

    pub fn add<C: Into<LayoutComponent>>(&mut self, component: C) {
        self.children.push(component.into());
    }
}

impl IntoTwilight<Vec<TwilightComponent>> for RootComponent {
    fn into_twilight(self) -> Vec<TwilightComponent> {
        let max_count = if self.require_components_v2() { 5 } else { 40 };
        let components_count = self.count();

        if components_count > max_count {
            panic!(
                "Maximum total number of components reached ({}/{})",
                components_count, max_count
            );
        }

        let mut id = HierarchicalId::new();

        if let Some(component_id) = self.component_id {
            id.set_prefix(component_id);
        }

        let mut twilight_components = Vec::new();

        for mut component in self.children {
            assign_ids(&mut component, &mut id);
            twilight_components.push(component.into_twilight());
            id.next_root();
        }

        twilight_components
    }
}