use twilight_model::channel::message::Component as TwilightComponent;

use crate::{models::components::{ComponentType, id::{HierarchicalId, IdAssignable}, layout::{action_row::ActionRow, container::Container, section::Section, separator::Separator}}, traits::component::{IntoComponent, IntoTwilight}};


pub mod action_row;
pub mod container;
pub mod separator;
pub mod section;

pub use crate::models::components::root::RootComponent;

pub enum LayoutComponent {
    ActionRow(ActionRow),
    Container(Container),
    Section(Section),
    Separator(Separator)
}

impl LayoutComponent {
    pub (crate) fn count(&self) -> usize {
        match self {
            Self::ActionRow(action_row) => action_row.count(),
            Self::Container(container) => container.count(),
            Self::Section(section) => section.count(),
            Self::Separator(_separator) => 1
        }
    }

    pub (crate) fn get_id(&self) -> i32 {
        match self {
            Self::ActionRow(action_row) => action_row.get_id(),
            Self::Container(container) => container.get_id(),
            Self::Section(section) => section.get_id(),
            Self::Separator(_) => 0
        }
    }

    pub (crate) fn require_components_v2(&self) -> bool {
        match self {
            Self::ActionRow(_) => false,
            Self::Container(_) => true,
            Self::Section(_) => true,
            Self::Separator(_) => true,
        }
    }
}

impl IdAssignable for LayoutComponent {
    fn set_id(&mut self, id: &HierarchicalId) {
        match self {
            Self::ActionRow(action_row) => action_row.set_id(id),
            Self::Container(container) => container.set_id(id),
            Self::Section(section) => section.set_id(id),
            Self::Separator(_separator) => {}
        }
    }

    fn children(&mut self) -> Box<dyn Iterator<Item = &mut dyn IdAssignable> + '_> {
        match self {
            Self::ActionRow(action_row) => action_row.children(),
            Self::Container(container) => container.children(),
            Self::Section(section) => section.children(),
            Self::Separator(_separator) => Box::new(std::iter::empty::<&mut dyn IdAssignable>())
        }
    }
}

impl From<Container> for LayoutComponent {
    fn from(value: Container) -> Self {
        Self::Container(value)
    }
}

impl From<ActionRow> for LayoutComponent {
    fn from(value: ActionRow) -> Self {
        Self::ActionRow(value)
    }
}

impl From<Separator> for LayoutComponent {
    fn from(value: Separator) -> Self {
        Self::Separator(value)
    }
}

impl From<Section> for LayoutComponent {
    fn from(value: Section) -> Self {
        Self::Section(value)
    }
}

impl IntoComponent for ActionRow {
    fn into_component(self) -> ComponentType {
        LayoutComponent::ActionRow(self).into_component()
    }
}

impl IntoComponent for Container {
    fn into_component(self) -> ComponentType {
        LayoutComponent::Container(self).into_component()
    }
}

impl IntoComponent for Section {
    fn into_component(self) -> ComponentType {
        LayoutComponent::Section(self).into_component()
    }
}

impl IntoComponent for Separator {
    fn into_component(self) -> ComponentType {
        LayoutComponent::Separator(self).into_component()
    }
}

impl IntoTwilight<TwilightComponent> for LayoutComponent {
    fn into_twilight(self) -> TwilightComponent {
        match self {
            Self::ActionRow(action_row) => action_row.into_twilight(),
            Self::Container(container) => container.into_twilight(),
            Self::Section(section) => section.into_twilight(),
            Self::Separator(separator) => separator.into_twilight()
        }
    }
}
