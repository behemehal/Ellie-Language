use ellie_core::bytecode::{RawType, TypeId};
use std::any::Any;

pub struct BytesObj {
    pub data: Vec<u8>,
    pub pos: usize,
}

pub enum HeapObject {
    Str(String),
    ClassInst(Vec<RawType>),
    Bytes(BytesObj),
    /// Opaque native-owned resource (TcpStream, UdpSocket, File, ...).
    /// The Box owns the value; dropping the heap entry runs its destructor.
    /// `None` means the handle was explicitly released via handle_drop.
    Handle(Option<Box<dyn Any + Send + Sync>>),
}

pub struct Heap {
    pub objects: Vec<HeapObject>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            objects: Vec::new(),
        }
    }

    fn zero_raw() -> RawType {
        RawType {
            type_id: TypeId::Int,
            size: 0,
            data: [0; 8],
        }
    }

    pub fn alloc_string(&mut self, s: String) -> usize {
        self.objects.push(HeapObject::Str(s));
        self.objects.len() - 1
    }

    pub fn alloc_class_inst(&mut self, n_fields: usize) -> usize {
        self.objects.push(HeapObject::ClassInst(vec![Self::zero_raw(); n_fields]));
        self.objects.len() - 1
    }

    pub fn alloc_bytes(&mut self, data: Vec<u8>) -> usize {
        self.objects.push(HeapObject::Bytes(BytesObj { data, pos: 0 }));
        self.objects.len() - 1
    }

    pub fn get_string(&self, idx: usize) -> Option<&str> {
        match self.objects.get(idx) {
            Some(HeapObject::Str(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_string_mut(&mut self, idx: usize) -> Option<&mut String> {
        match self.objects.get_mut(idx) {
            Some(HeapObject::Str(s)) => Some(s),
            _ => None,
        }
    }

    pub fn get_field(&self, idx: usize, field_idx: usize) -> Option<RawType> {
        match self.objects.get(idx) {
            Some(HeapObject::ClassInst(fields)) => fields.get(field_idx).copied(),
            _ => None,
        }
    }

    pub fn set_field(&mut self, idx: usize, field_idx: usize, val: RawType) {
        if let Some(HeapObject::ClassInst(fields)) = self.objects.get_mut(idx) {
            if field_idx < fields.len() {
                fields[field_idx] = val;
            }
        }
    }

    // ── Bytes accessors ────────────────────────────────────────────────────

    pub fn is_bytes(&self, idx: usize) -> bool {
        matches!(self.objects.get(idx), Some(HeapObject::Bytes(_)))
    }

    pub fn bytes_len(&self, idx: usize) -> Option<usize> {
        match self.objects.get(idx) {
            Some(HeapObject::Bytes(b)) => Some(b.data.len()),
            _ => None,
        }
    }

    pub fn bytes_pos(&self, idx: usize) -> Option<usize> {
        match self.objects.get(idx) {
            Some(HeapObject::Bytes(b)) => Some(b.pos),
            _ => None,
        }
    }

    pub fn bytes_set_pos(&mut self, idx: usize, pos: usize) -> bool {
        if let Some(HeapObject::Bytes(b)) = self.objects.get_mut(idx) {
            b.pos = pos.min(b.data.len());
            true
        } else {
            false
        }
    }

    pub fn bytes_get(&self, idx: usize, i: usize) -> Option<u8> {
        match self.objects.get(idx) {
            Some(HeapObject::Bytes(b)) => b.data.get(i).copied(),
            _ => None,
        }
    }

    pub fn bytes_set(&mut self, idx: usize, i: usize, v: u8) -> bool {
        if let Some(HeapObject::Bytes(b)) = self.objects.get_mut(idx) {
            if let Some(slot) = b.data.get_mut(i) {
                *slot = v;
                return true;
            }
        }
        false
    }

    pub fn bytes_view(&self, idx: usize) -> Option<&[u8]> {
        match self.objects.get(idx) {
            Some(HeapObject::Bytes(b)) => Some(b.data.as_slice()),
            _ => None,
        }
    }

    pub fn bytes_clone(&self, idx: usize) -> Option<Vec<u8>> {
        match self.objects.get(idx) {
            Some(HeapObject::Bytes(b)) => Some(b.data.clone()),
            _ => None,
        }
    }

    /// Mutable slice into a bytes object at the given absolute (offset, len).
    /// Returns None if idx is not a bytes object or [offset, offset+len) is out of bounds.
    /// The slice is valid until the next heap operation that may resize the vec.
    pub fn bytes_mut_slice(
        &mut self,
        idx: usize,
        offset: usize,
        len: usize,
    ) -> Option<&mut [u8]> {
        if let Some(HeapObject::Bytes(b)) = self.objects.get_mut(idx) {
            let end = offset.checked_add(len)?;
            if end > b.data.len() {
                return None;
            }
            Some(&mut b.data[offset..end])
        } else {
            None
        }
    }

    pub fn bytes_append(&mut self, idx: usize, src: &[u8]) -> bool {
        if let Some(HeapObject::Bytes(b)) = self.objects.get_mut(idx) {
            b.data.extend_from_slice(src);
            true
        } else {
            false
        }
    }

    // ── Handle accessors ───────────────────────────────────────────────────

    pub fn is_handle(&self, idx: usize) -> bool {
        matches!(self.objects.get(idx), Some(HeapObject::Handle(_)))
    }

    pub fn alloc_handle(&mut self, value: Box<dyn Any + Send + Sync>) -> usize {
        self.objects.push(HeapObject::Handle(Some(value)));
        self.objects.len() - 1
    }

    pub fn handle_view(&self, idx: usize) -> Option<&(dyn Any + Send + Sync)> {
        match self.objects.get(idx) {
            Some(HeapObject::Handle(Some(b))) => Some(b.as_ref()),
            _ => None,
        }
    }

    pub fn handle_view_mut(&mut self, idx: usize) -> Option<&mut (dyn Any + Send + Sync)> {
        match self.objects.get_mut(idx) {
            Some(HeapObject::Handle(Some(b))) => Some(b.as_mut()),
            _ => None,
        }
    }

    /// Take ownership of the boxed value, leaving an empty handle slot behind.
    /// Useful when a native wrapper needs to consume-and-replace the resource.
    pub fn handle_take(&mut self, idx: usize) -> Option<Box<dyn Any + Send + Sync>> {
        match self.objects.get_mut(idx) {
            Some(HeapObject::Handle(slot)) => slot.take(),
            _ => None,
        }
    }

    /// Drop the resource explicitly. Subsequent handle_view/handle_view_mut return None.
    pub fn handle_drop(&mut self, idx: usize) -> bool {
        if let Some(HeapObject::Handle(slot)) = self.objects.get_mut(idx) {
            slot.take();
            true
        } else {
            false
        }
    }
}
