// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use anyhow::Error;
use yew::Html;

use super::{context::Context, subscribe::FromDelta};
use crate::states::{local_state::LocalState, remote_state::RemoteState};

pub trait Widget: Sized + 'static {
    type Msg: FromDelta<LocalState> + FromDelta<RemoteState> + Send;

    fn create(ctx: &mut Context<Self>) -> Self;

    fn initialize(&mut self, _ctx: &mut Context<Self>) -> Result<(), Error> {
        Ok(())
    }

    fn on_event(&mut self, _msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.redraw();
        Ok(())
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.view_opt(ctx).unwrap_or_else(|| self.view_empty())
    }

    fn view_empty(&self) -> Html {
        Html::default()
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html>;
}
