#[cfg(feature = "dx11")]
use std::cell::Cell;

use windows::core::{Interface, Result};
use windows::Win32::Graphics::Dxgi::{IDXGISwapChain, IDXGISwapChain2};

#[cfg(feature = "dx11")]
thread_local! {
    static IN_PRESENT: Cell<bool> = const { Cell::new(false) };
}

#[cfg(feature = "dx11")]
pub(super) struct PresentGuard;

#[cfg(feature = "dx11")]
impl PresentGuard {
    pub(super) fn enter() -> Option<Self> {
        IN_PRESENT.with(|active| if active.replace(true) { None } else { Some(Self) })
    }
}

#[cfg(feature = "dx11")]
impl Drop for PresentGuard {
    fn drop(&mut self) {
        IN_PRESENT.with(|active| active.set(false));
    }
}

pub(super) fn display_size(swap_chain: &IDXGISwapChain) -> Result<(u32, u32)> {
    if let Ok(swap_chain2) = swap_chain.cast::<IDXGISwapChain2>() {
        let (mut width, mut height) = (0, 0);
        if unsafe { swap_chain2.GetSourceSize(&mut width, &mut height) }.is_ok() {
            return Ok((width, height));
        }
    }
    let desc = unsafe { swap_chain.GetDesc()? };
    Ok((desc.BufferDesc.Width, desc.BufferDesc.Height))
}
