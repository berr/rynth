use std::marker::PhantomData;

#[derive(Copy, Clone, PartialEq)]
pub struct ComponentId<T>(pub usize, PhantomData<T>);

impl<T> ComponentId<T> {
    fn new(id: usize) -> Self {
        ComponentId(id, PhantomData)
    }
}

struct StoredComponent<T: ?Sized, Id> {
    id: ComponentId<Id>,
    data: Box<T>,
}

pub struct ComponentsStore<T: ?Sized, Id> {
    components: Vec<StoredComponent<T, Id>>,
    created_elements: usize,
}

impl<T: ?Sized, Id> Default for ComponentsStore<T, Id> {
    fn default() -> Self {
        Self {
            components: vec![],
            created_elements: 0,
        }
    }
}

impl<T: ?Sized, Id: Copy + Clone + PartialEq> ComponentsStore<T, Id> {
    pub fn add_component(&mut self, component: Box<T>) -> ComponentId<Id> {
        let id = ComponentId::new(self.created_elements);
        self.components.push(StoredComponent {
            id,
            data: component,
        });
        self.created_elements += 1;
        id
    }

    pub fn get_component(&self, id: ComponentId<Id>) -> Option<&T> {
        self.components
            .iter()
            .find(|node| node.id == id)
            .map(|m| m.data.as_ref())
    }

    pub fn get_component_mut(&mut self, id: ComponentId<Id>) -> Option<&mut T> {
        self.components
            .iter_mut()
            .find(|node| node.id == id)
            .map(|m| m.data.as_mut())
    }

    pub fn iter_components(&self) -> impl Iterator<Item = &T> {
        self.components.iter().map(|n| n.data.as_ref())
    }

    pub fn iter_components_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.components.iter_mut().map(|n| n.data.as_mut())
    }
}
