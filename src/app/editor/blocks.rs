//! The Types and associated functions for Blocks
//!
//! This module defines, what blocks are available, do and look like. Interaction with other
//! elements is handled in [`editor`](crate::app::editor) itself.

use leptos::prelude::*;

use super::{UnReStack, UnReStep};

const TEXTAREA_DEFAULT_ROWS: i32 = 2;
const TEXTAREA_DEFAULT_COLS: i32 = 30;

#[derive(Debug, Clone)]
pub(super) struct EditorBlock {
    id: i32,
    inner: InnerBlock,
    focus_on_load: bool,
}
#[component]
fn InnerView(inner: InnerBlock, id: i32, focus_on_load: bool) -> impl IntoView {
    let focus_element = NodeRef::new();
    // if do_focus is true, focus this input when it is created
    if focus_on_load {
        Effect::new(move |_| {
            focus_element.on_load(|input: web_sys::HtmlTextAreaElement| {
                let _ = input.focus();
            });
        });
    }

    let undo_stack = use_context::<RwSignal<UnReStack>>()
        .expect("Blocks need to be nested in an editor providing an undo stack");

    match inner {
        InnerBlock::Text(content) => {
            // initialize the old content with the current one
            let (old_content, set_old_content) = signal(content.get_untracked());
            view! {
                    <div>
                        <p
                            class="font-light text-xs">
                            "Raw Text: "
                        </p>
                        <textarea
                        class="bg-yellow-100 text-black font-mono"
                        id={format!("block-input-{id}")}
                        node_ref=focus_element
                        autocomplete="false"
                        spellcheck="false"
                        rows=TEXTAREA_DEFAULT_ROWS
                        cols=TEXTAREA_DEFAULT_COLS
                        prop:value=content
                        on:input:target=move |ev| {
                            //change the current content when updated
                            content.set(ev.target().value());
                        }
                        on:change:target=move |ev| {
                            // the input is unfocused - we now want to add something to the undo
                            // machine
                            // the content that was last saved (on last unfocus of this element)
                            let current_old_content = old_content.get();
                            // current real value
                            let new_content = ev.target().value();
                            // save the new content on this unfocus (for the next run of this
                            // closure)
                            set_old_content.set(new_content.clone());
                            // add the diff between the last unfocus and this unfocus to the stack
                            undo_stack.write().push_undo(UnReStep::new_data_change(id, InnerBlockDry::Text(current_old_content), InnerBlockDry::Text(new_content)));
                        }
                    />
                    </div>
                }.into_any()
        }
        InnerBlock::Lacuna(content, reason) => {
            let (old_content, set_old_content) = signal(content.get_untracked());
            let (old_reason, set_old_reason) = signal(reason.get_untracked());
            view! {
                    <div>
                        <span
                            class="font-light text-xs">
                                "Lacuna because of "
                        </span>
                        <input
                        prop:value=reason
                        class="text-sm"
                        placeholder="reason"
                        autocomplete="false"
                        spellcheck="false"
                        on:input:target=move |ev| {
                            reason.set(ev.target().value());
                        }
                        on:change:target=move |ev| {
                            let current_old_reason = old_reason.get();
                            let new_reason = ev.target().value();
                            set_old_reason.set(new_reason.clone());
                            undo_stack.write().push_undo(UnReStep::new_data_change(id, InnerBlockDry::Lacuna(content.get(), current_old_reason), InnerBlockDry::Lacuna(content.get(), new_reason)));
                        }/>
                        <span
                            class="font-light text-xs">
                            :
                        </span>
                        <br/>
                        <textarea
                        class="bg-orange-100 text-black font-mono"
                        id={format!("block-input-{id}")}
                        node_ref=focus_element
                        prop:value=content
                        autocomplete="false"
                        spellcheck="false"
                        rows=TEXTAREA_DEFAULT_ROWS
                        cols=TEXTAREA_DEFAULT_COLS
                        on:input:target=move |ev| {
                            content.set(ev.target().value());
                        }
                        on:change:target=move |ev| {
                            let current_old_content = old_content.get();
                            let new_content = ev.target().value();
                            set_old_content.set(new_content.clone());
                            undo_stack.write().push_undo(UnReStep::new_data_change(id, InnerBlockDry::Lacuna(current_old_content, reason.get()), InnerBlockDry::Lacuna(new_content, reason.get())));
                        }
                    />
                    </div>
                }.into_any()
        }
        InnerBlock::Uncertain(content, reason) => {
            let (old_content, set_old_content) = signal(content.get_untracked());
            let (old_reason, set_old_reason) = signal(reason.get_untracked());
            view! {
                    <div>
                        <span
                            class="font-light text-xs">
                            "Uncertain because of "
                        </span>
                        <input
                        class="text-sm"
                        placeholder="reason"
                        autocomplete="false"
                        spellcheck="false"
                        prop:value=reason
                        on:input:target=move |ev| {
                            reason.set(ev.target().value());
                        }
                        on:change:target=move |ev| {
                            let current_old_reason = old_reason.get();
                            let new_reason = ev.target().value();
                            set_old_reason.set(new_reason.clone());
                            undo_stack.write().push_undo(UnReStep::new_data_change(id, InnerBlockDry::Uncertain(content.get(), current_old_reason), InnerBlockDry::Uncertain(content.get(), new_reason)));
                        }/>
                        <span class="font-light text-xs">
                            :
                        </span>
                        <br/>
                        <textarea
                        id={format!("block-input-{id}")}
                        class="bg-orange-100 text-black font-mono"
                        node_ref=focus_element
                        autocomplete="false"
                        spellcheck="false"
                        rows=TEXTAREA_DEFAULT_ROWS
                        cols=TEXTAREA_DEFAULT_COLS
                        prop:value=content
                        on:input:target=move |ev| {
                            content.set(ev.target().value());
                        }
                        on:change:target=move |ev| {
                            let current_old_content = old_content.get();
                            let new_content = ev.target().value();
                            set_old_content.set(new_content.clone());
                            undo_stack.write().push_undo(UnReStep::new_data_change(id, InnerBlockDry::Uncertain(current_old_content, reason.get()), InnerBlockDry::Uncertain(new_content, reason.get())));
                        }
                    />
                    </div>
                }.into_any()
        }
        InnerBlock::Break(reason) => {
            let (old_reason, set_old_reason) = signal(reason.get_untracked());
            view! {
                    <div>
                        <p
                            class="font-light text-xs">
                            "Break: "
                        </p>
                        // TODO make this a drop down instead
                        <input
                        id={format!("block-input-{id}")}
                        autocomplete="false"
                        spellcheck="false"
                        prop:value=reason
                        on:input:target=move |ev| {
                            reason.set(ev.target().value());
                        }
                        on:change:target=move |ev| {
                            let current_old_reason = old_reason.get();
                            // current real value
                            let new_reason = ev.target().value();
                            set_old_reason.set(new_reason.clone());
                            // add the diff between the last unfocus and this unfocus to the stack
                            undo_stack.write().push_undo(UnReStep::new_data_change(id, InnerBlockDry::Break(current_old_reason), InnerBlockDry::Break(new_reason)));
                        }
                    />
                    </div>
                }.into_any()
        }
    }
}

impl EditorBlock {
    // construct a block with id, type, content, and focus state
    pub fn new(id: i32, block_type: InnerBlockType, content: String, focus_on_load: bool) -> Self {
        Self {
            id,
            inner: InnerBlock::new_from_type_and_content(block_type, content),
            focus_on_load,
        }
    }

    /// Get this blocks id
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Display this block
    pub(super) fn view(self) -> impl IntoView {
        view! {
            <span>
                // we probably do not want to show the blocks ID to the user
                // {self.id}
                // ":"
                <InnerView inner=self.inner id=self.id focus_on_load=self.focus_on_load></InnerView></span>
        }
    }

    pub(super) fn set_autoload(&mut self, focus_on_load: bool) {
        self.focus_on_load = focus_on_load
    }

    /// Overwrite the inner block with `new_inner` if it is currently `old_inner`
    ///
    /// Will clone new_inner if required, but not if the assert failed
    pub(super) fn overwrite_inner(
        &mut self,
        old_inner: &InnerBlockDry,
        new_inner: &InnerBlockDry,
    ) -> Option<()> {
        if *old_inner != self.inner {
            None
        } else {
            self.inner.overwrite_with(new_inner.clone());
            Some(())
        }
    }

    /// Split this block, returning new blocks and the index of the block which defaults as the
    /// newly inserted one.
    pub(super) fn split_at_selection(
        &self,
        start: usize,
        end: usize,
        new_block_type: InnerBlockType,
        new_index: &mut i32,
    ) -> Vec<EditorBlock> {
        // add the ids to the inner blocks created from splitting this inner block
        self.inner
            .split_at_selection(start, end, new_block_type)
            .into_iter()
            .map(|iblck| {
                let block = EditorBlock {
                    id: *new_index,
                    inner: iblck.0,
                    focus_on_load: iblck.1,
                };
                *new_index += 1;
                return block;
            })
            .collect()
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EditorBlockDry {
    id: i32,
    inner: InnerBlockDry,
    focus_on_load: bool,
}
impl EditorBlockDry {
    // construct a block with id, type, content, and focus state
    pub fn new(id: i32, block_type: InnerBlockType, content: String, focus_on_load: bool) -> Self {
        Self {
            id,
            inner: InnerBlockDry::new_from_type_and_content(block_type, content),
            focus_on_load,
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
}
/// Dehydrate an [`EditorBlock`]
impl From<EditorBlock> for EditorBlockDry {
    fn from(value: EditorBlock) -> Self {
        Self {
            id: value.id,
            inner: value.inner.into(),
            focus_on_load: value.focus_on_load,
        }
    }
}
/// Hydrate a dry [`EditorBlockDry`].
impl From<EditorBlockDry> for EditorBlock {
    fn from(value: EditorBlockDry) -> Self {
        Self {
            id: value.id,
            inner: value.inner.into(),
            focus_on_load: value.focus_on_load,
        }
    }
}
impl PartialEq<EditorBlock> for EditorBlockDry {
    fn eq(&self, other: &EditorBlock) -> bool {
        self.id == other.id && self.inner == other.inner
    }
}
impl PartialEq<EditorBlockDry> for EditorBlock {
    fn eq(&self, other: &EditorBlockDry) -> bool {
        self.id == other.id && self.inner == other.inner
    }
}

/// Dataless types for Blocks
pub(super) enum InnerBlockType {
    /// Raw text without special markup
    Text,
    /// A part of Text with uncertainty
    ///
    /// These are (sequences of) glyphs where the intention is not clear
    Uncertain,
    /// A part of Text that is absent or entirely unreadable
    ///
    /// These are places that can only be supplied, no actual reading of the remains is possible
    Lacuna,
    /// A break (Line, Column, Page, ...)
    Break,
}
/// Block type with data
///
/// NOTE: this could also be done with Traits and generic functions.
/// That would be nicer in a sense, but we are compiling into WASM, so binary size is more
/// important then nice generics imho. I keep it as this enum with some runtimechecks.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) enum InnerBlock {
    /// Raw text without special markup
    /// text
    Text(RwSignal<String>),
    /// A part of Text with uncertainty
    /// (proposed-text, reason)
    Uncertain(RwSignal<String>, RwSignal<String>),
    /// A part of Text that is absent or entirely unreadable
    /// (proposed-text, reason)
    Lacuna(RwSignal<String>, RwSignal<String>),
    /// A break (Line, Column, Page, ...)
    /// TODO: we want this to be an enum over type instead; with selection menu in GUI
    /// (type of break)
    Break(RwSignal<String>),
}
impl InnerBlock {
    /// overwrite own data with that given from new_block, but only if the types are the same
    fn overwrite_with(&mut self, new_block: InnerBlockDry) {
        match self {
            Self::Text(x) => match new_block {
                InnerBlockDry::Text(y) => {
                    *x.write() = y;
                }
                _ => {}
            },
            Self::Break(x) => match new_block {
                InnerBlockDry::Break(y) => {
                    *x.write() = y;
                }
                _ => {}
            },
            Self::Lacuna(x, y) => match new_block {
                InnerBlockDry::Lacuna(a, b) => {
                    *x.write() = a;
                    *y.write() = b;
                }
                _ => {}
            },
            Self::Uncertain(x, y) => match new_block {
                InnerBlockDry::Uncertain(a, b) => {
                    *x.write() = a;
                    *y.write() = b;
                }
                _ => {}
            },
        }
    }

    /// Copy the metadata from [`self`] but get the content from another string
    pub fn clone_with_new_content(&self, content: String) -> Self {
        match self {
            Self::Text(_) => InnerBlock::Text(RwSignal::new(content.to_owned())),
            Self::Uncertain(_, y) => InnerBlock::Uncertain(RwSignal::new(content.to_owned()), *y),
            Self::Lacuna(_, y) => InnerBlock::Lacuna(RwSignal::new(content.to_owned()), *y),
            Self::Break(y) => InnerBlock::Break(*y),
        }
    }

    /// get this blocks content if this blocktype has content
    ///
    /// This is one of the functions which would be nicer with Traits, but here we need to return
    /// Option instead.
    pub fn content(&self) -> Option<guards::ReadGuard<String, guards::Plain<String>>> {
        match &self {
            Self::Text(el) => Some(el.read()),
            Self::Uncertain(el, _) => Some(el.read()),
            Self::Lacuna(el, _) => Some(el.read()),
            Self::Break(_) => None,
        }
    }

    /// Create a new Block with content
    pub fn new_from_type_and_content(block_type: InnerBlockType, content: String) -> Self {
        InnerBlockDry::new_from_type_and_content(block_type, content).into()
    }

    /// Create a new Block without content
    pub(super) fn new_from_type(block_type: InnerBlockType) -> Self {
        Self::new_from_type_and_content(block_type, String::default())
    }

    /// Split this block, returning new blocks and the index of the block which defaults as the
    /// newly inserted one.
    ///
    /// Returns a vec of InnerBlock, focus_on_load
    fn split_at_selection(
        &self,
        start: usize,
        end: usize,
        new_block_type: InnerBlockType,
    ) -> Vec<(InnerBlock, bool)> {
        let complete_value = match self.content() {
            Some(x) => x,
            // Block types without content can never fire split_at_selection,
            // so the function should return itself
            None => {
                return vec![(self.clone(), false)];
            }
        };
        let (before_part, new_part, after_part) = if start == 0 {
            if end == complete_value.len() {
                // everything selected - do nothing
                (None, complete_value.as_ref(), None)
            } else {
                // create a new node before
                (None, &complete_value[0..end], Some(&complete_value[end..]))
            }
        } else {
            if end == complete_value.len() {
                // create a new node after
                (
                    Some(&complete_value[0..start]),
                    &complete_value[start..],
                    None,
                )
            } else {
                // split in three
                (
                    Some(&complete_value[..start]),
                    &complete_value[start..end],
                    Some(&complete_value[end..]),
                )
            }
        };
        let mut res = vec![];
        // first and last block (if any) keeps the same type as this one
        if let Some(content) = before_part {
            res.push((self.clone_with_new_content(content.to_owned()), false));
        };
        res.push((
            InnerBlock::new_from_type_and_content(new_block_type, new_part.to_owned()),
            // we do want to autofocus on the middle block
            true,
        ));
        if let Some(content) = after_part {
            res.push((self.clone_with_new_content(content.to_owned()), false));
        };
        return res;
    }
}

/// Version of [`InnerBlock`] without Signals
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub(super) enum InnerBlockDry {
    /// Raw text without special markup
    /// text
    Text(String),
    /// A part of Text with uncertainty
    /// (proposed-text, reason)
    Uncertain(String, String),
    /// A part of Text that is absent or entirely unreadable
    /// (proposed-text, reason)
    Lacuna(String, String),
    /// A break (Line, Column, Page, ...)
    /// TODO: we want this to be an enum over type instead; with selection menu in GUI
    /// (type of break)
    Break(String),
}
impl InnerBlockDry {
    /// Create a new Block with content
    pub fn new_from_type_and_content(block_type: InnerBlockType, content: String) -> Self {
        match block_type {
            InnerBlockType::Text => InnerBlockDry::Text(content),
            InnerBlockType::Uncertain => InnerBlockDry::Uncertain(content, String::default()),
            InnerBlockType::Lacuna => InnerBlockDry::Lacuna(content, String::default()),
            InnerBlockType::Break => {
                // Breaks do not have content; ignore it
                InnerBlockDry::Break(String::default())
            }
        }
    }
}
/// Dehydrate [`InnerBlock`]
impl From<InnerBlock> for InnerBlockDry {
    fn from(value: InnerBlock) -> Self {
        match value {
            InnerBlock::Break(x) => InnerBlockDry::Break(x.get()),
            InnerBlock::Text(x) => InnerBlockDry::Text(x.get()),
            InnerBlock::Lacuna(x, y) => InnerBlockDry::Lacuna(x.get(), y.get()),
            InnerBlock::Uncertain(x, y) => InnerBlockDry::Uncertain(x.get(), y.get()),
        }
    }
}
/// Hydrate [`InnerBlockDry`]
impl From<InnerBlockDry> for InnerBlock {
    fn from(value: InnerBlockDry) -> Self {
        match value {
            InnerBlockDry::Break(x) => InnerBlock::Break(RwSignal::new(x)),
            InnerBlockDry::Text(x) => InnerBlock::Text(RwSignal::new(x)),
            InnerBlockDry::Lacuna(x, y) => InnerBlock::Lacuna(RwSignal::new(x), RwSignal::new(y)),
            InnerBlockDry::Uncertain(x, y) => {
                InnerBlock::Uncertain(RwSignal::new(x), RwSignal::new(y))
            }
        }
    }
}
/// Compare [`InnerBlock`] agains [`InnerBlockDry`] without (de-)hydrating either side
///
/// This is more efficient because we do not need to clone or create a signal
impl PartialEq<InnerBlock> for InnerBlockDry {
    fn eq(&self, other: &InnerBlock) -> bool {
        match self {
            InnerBlockDry::Break(x) => match other {
                InnerBlock::Break(y) => *x == *y.read(),
                _ => false,
            },
            InnerBlockDry::Text(x) => match other {
                InnerBlock::Text(y) => *x == *y.read(),
                _ => false,
            },
            InnerBlockDry::Uncertain(x, y) => match other {
                InnerBlock::Uncertain(a, b) => *x == *a.read() && *y == *b.read(),
                _ => false,
            },
            InnerBlockDry::Lacuna(x, y) => match other {
                InnerBlock::Lacuna(a, b) => *x == *a.read() && *y == *b.read(),
                _ => false,
            },
        }
    }
}
/// Compare [`InnerBlockDry`] agains [`InnerBlock`] without (de-)hydrating either side
///
/// This is more efficient because we do not need to clone or create a signal
impl PartialEq<InnerBlockDry> for InnerBlock {
    fn eq(&self, other: &InnerBlockDry) -> bool {
        other.eq(self)
    }
}
