// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Implementation of the rendering tree.

// external
use ego_tree;
use svgdom;

// self
pub use self::node::*;
pub use self::attribute::*;

mod attribute;
mod dump;
mod node;


/// Alias for ego_tree::NodeId<NodeKind>.
pub type NodeId = ego_tree::NodeId<NodeKind>;

/// Alias for ego_tree::NodeRef<NodeKind>.
pub type NodeRef<'a> = ego_tree::NodeRef<'a, NodeKind>;

/// A rendering tree container.
///
/// Contains all the nodes that are required for rendering.
pub struct RenderTree(ego_tree::Tree<NodeKind>);

impl RenderTree {
    /// Creates a new `RenderTree`.
    pub fn new(svg: Svg) -> Self {
        let mut tree = ego_tree::Tree::new(NodeKind::Svg(svg));
        tree.root_mut().append(NodeKind::Defs);
        RenderTree(tree)
    }

    /// Returns the root node.
    pub fn root(&self) -> NodeRef {
        self.0.root()
    }

    /// Returns the `Svg` node's data.
    pub fn svg_node(&self) -> &Svg {
        if let NodeKind::Svg(ref svg) = *self.root().value() {
            svg
        } else {
            unreachable!();
        }
    }

    /// Returns the `Defs` node.
    pub fn defs(&self) -> NodeRef {
        self.root().first_child().unwrap()
    }

    pub(crate) fn append_defs(&mut self, kind: NodeKind) -> NodeId {
        let defs_id = self.defs().id();
        self.append_child(defs_id, kind)
    }

    pub(crate) fn append_child(&mut self, parent: NodeId, kind: NodeKind) -> NodeId {
        let mut parent = self.0.get_mut(parent);
        parent.append(kind).id()
    }

    pub(crate) fn defs_at(&self, id: NodeId) -> NodeRef {
        for n in self.defs().children() {
            if n.id() == id {
                return n;
            }
        }

        unreachable!();
    }

    pub(crate) fn defs_id(&self, id: &str) -> Option<NodeId> {
        for n in self.defs().children() {
            if n.svg_id() == id {
                return Some(n.id());
            }
        }

        unreachable!();
    }

    /// Converts the document to `svgdom::Document`.
    ///
    /// Used to save document to file for debug purposes.
    pub fn to_svgdom(&self) -> svgdom::Document {
        dump::conv_doc(self)
    }
}


/// Additional `NodeRef` method.
pub trait NodeExt {
    /// Returns node's ID.
    ///
    /// If a current node doesn't support ID - an empty string
    /// will be returned.
    fn svg_id(&self) -> &str;

    /// Returns node's transform.
    ///
    /// If a current node doesn't support transformation - a default
    /// transform will be returned.
    fn transform(&self) -> Transform;
}

impl<'a> NodeExt for NodeRef<'a> {
    fn svg_id(&self) -> &str {
        self.value().id()
    }

    fn transform(&self) -> Transform {
        self.value().transform()
    }
}