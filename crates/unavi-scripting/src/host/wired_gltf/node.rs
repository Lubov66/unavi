use wasm_bridge::component::Resource;

use crate::state::StoreState;

use super::{
    bindgen::wired::gltf::{
        mesh::Mesh,
        node::{Host, HostNode, Node, Transform},
    },
    WiredGltfAction,
};

impl HostNode for StoreState {
    fn id(&mut self, self_: Resource<Node>) -> wasm_bridge::Result<u32> {
        Ok(self_.rep())
    }

    fn name(&mut self, self_: Resource<Node>) -> wasm_bridge::Result<String> {
        let node = self.table.get(&self_)?;
        Ok(node.name.clone())
    }
    fn set_name(&mut self, self_: Resource<Node>, value: String) -> wasm_bridge::Result<()> {
        let node = self.table.get_mut(&self_)?;
        node.name = value;
        Ok(())
    }

    fn mesh(&mut self, self_: Resource<Node>) -> wasm_bridge::Result<Option<Resource<Mesh>>> {
        let node = self.table.get(&self_)?;
        Ok(node.mesh.map(Resource::new_own))
    }
    fn set_mesh(
        &mut self,
        self_: Resource<Node>,
        value: Option<Resource<Mesh>>,
    ) -> wasm_bridge::Result<()> {
        let node = self.table.get_mut(&self_)?;
        node.mesh = value.map(|v| v.rep());

        self.sender.send(WiredGltfAction::SetNodeMesh {
            id: self_.rep(),
            mesh: node.mesh,
        })?;

        Ok(())
    }

    fn parent(&mut self, self_: Resource<Node>) -> wasm_bridge::Result<Option<Resource<Node>>> {
        let node = self.table.get(&self_)?;
        Ok(node.parent.map(Resource::new_own))
    }
    fn children(&mut self, self_: Resource<Node>) -> wasm_bridge::Result<Vec<Resource<Node>>> {
        let node = self.table.get_mut(&self_)?;
        Ok(node
            .children
            .iter()
            .map(|rep| Resource::new_own(*rep))
            .collect())
    }
    fn add_child(
        &mut self,
        self_: Resource<Node>,
        value: Resource<Node>,
    ) -> wasm_bridge::Result<()> {
        let rep = self_.rep();

        // Add child to children.
        let node = self.table.get_mut(&self_)?;
        node.children.insert(value.rep());

        // Remove child from old parent's children.
        let child = self.table.get(&value)?;
        if let Some(parent_rep) = child.parent {
            let parent_res = Resource::new_own(parent_rep);
            self.remove_child(parent_res, self_)?;
        }

        // Set parent.
        let child = self.table.get_mut(&value)?;
        child.parent = Some(rep);

        self.sender.send(WiredGltfAction::SetNodeParent {
            id: value.rep(),
            parent: child.parent,
        })?;

        Ok(())
    }
    fn remove_child(
        &mut self,
        self_: Resource<Node>,
        value: Resource<Node>,
    ) -> wasm_bridge::Result<()> {
        let node = self.table.get_mut(&self_)?;
        node.children.remove(&value.rep());

        self.sender.send(WiredGltfAction::SetNodeParent {
            id: self_.rep(),
            parent: None,
        })?;

        Ok(())
    }

    fn transform(&mut self, self_: Resource<Node>) -> wasm_bridge::Result<Resource<Transform>> {
        let node = self.table.get(&self_)?;
        Ok(Resource::new_own(node.transform.rep()))
    }

    fn drop(&mut self, _rep: Resource<Node>) -> wasm_bridge::Result<()> {
        Ok(())
    }
}

impl Host for StoreState {
    fn list_nodes(&mut self) -> wasm_bridge::Result<Vec<Resource<Node>>> {
        Ok(self
            .nodes
            .iter()
            .map(|res| Resource::new_own(res.rep()))
            .collect())
    }

    fn create_node(&mut self) -> wasm_bridge::Result<Resource<Node>> {
        let node = Node::try_new(&mut self.table)?;
        let resource = self.table.push(node)?;
        let node_rep = resource.rep();
        self.nodes.push(resource);

        self.sender
            .send(WiredGltfAction::CreateNode { id: node_rep })?;

        Ok(Resource::new_own(node_rep))
    }

    fn remove_node(&mut self, value: Resource<Node>) -> wasm_bridge::Result<()> {
        let rep = value.rep();
        self.table.delete(value)?;

        let index =
            self.nodes
                .iter()
                .enumerate()
                .find_map(|(i, item)| if item.rep() == rep { Some(i) } else { None });
        if let Some(index) = index {
            self.nodes.remove(index);
        }

        self.sender.send(WiredGltfAction::RemoveNode { id: rep })?;

        Ok(())
    }
}
