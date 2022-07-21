use super::{handler::*, handler_context::*, pipeline::*};

use log::warn;
use std::error::Error;
use std::marker::PhantomData;
use std::rc::Weak;

pub trait PipelineContext: Sized {
    fn get_direction(&self) -> HandlerDir;
    fn attach_pipeline(&mut self);
    fn detach_pipeline(&mut self);
    fn set_next_in(&mut self, _ctx: Option<Self>) {}
    fn set_next_out(&mut self, _ctx: Option<Self>) {}
}

pub trait InboundLink<In> {
    fn read(&mut self, msg: In);
    fn read_eof(&mut self);
    fn read_error(&mut self, err: Box<dyn Error>);
    fn transport_active(&mut self);
    fn transport_inactive(&mut self);
}

pub trait OutboundLink<Out> {
    fn write(&mut self, msg: Out);
    fn write_error(&mut self, err: Box<dyn Error>);
    fn close(&mut self);
}

pub struct InboundContextImpl<Rin, Rout, Context, H, Link>
where
    Context: InboundHandlerContext<Rout>,
    H: InboundHandler<Rin, Rout, Context>,
    Link: InboundLink<Rout>,
{
    ctx: Option<Context>,
    pipeline: Weak<PipelineBase>,
    handler: H,
    next_in: Option<Link>,
    attached: bool,
    phantom_rin: PhantomData<Rin>,
    phantom_rout: PhantomData<Rout>,
}

impl<Rin, Rout, Context, H, Link> InboundContextImpl<Rin, Rout, Context, H, Link>
where
    Context: InboundHandlerContext<Rout>,
    H: InboundHandler<Rin, Rout, Context>,
    Link: InboundLink<Rout>,
{
    pub fn get_handler(&self) -> &H {
        &self.handler
    }

    pub fn initialize(&mut self, pipeline: Weak<PipelineBase>, handler: H) {
        self.pipeline = pipeline;
        self.handler = handler;
    }
}

impl<Rin, Rout, Context, H, Link> PipelineContext
    for InboundContextImpl<Rin, Rout, Context, H, Link>
where
    Context: InboundHandlerContext<Rout>,
    H: InboundHandler<Rin, Rout, Context>,
    Link: InboundLink<Rout>,
{
    fn get_direction(&self) -> HandlerDir {
        self.handler.get_direction()
    }

    fn attach_pipeline(&mut self) {
        if !self.attached {
            self.handler.attach_context(self.ctx.as_ref());
            self.attached = true;
        }
    }

    fn detach_pipeline(&mut self) {
        self.attached = false;
        self.handler.detach_context();
    }

    fn set_next_in(&mut self, ctx: Option<Self>) {
        //TODO:
        if ctx.is_none() {
            self.next_in = None;
            return;
        }
        //self.next_in = ctx;
    }
}

impl<Rin, Rout, Context, H, Link> InboundHandlerContext<Rout>
    for InboundContextImpl<Rin, Rout, Context, H, Link>
where
    Context: InboundHandlerContext<Rout>,
    H: InboundHandler<Rin, Rout, Context>,
    Link: InboundLink<Rout>,
{
    fn fire_read(&mut self, msg: Rout) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.read(msg);
        } else {
            warn!("read reached end of pipeline");
        }
    }

    fn fire_read_eof(&mut self) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.read_eof();
        } else {
            warn!("read_eof reached end of pipeline");
        }
    }

    fn fire_read_error(&mut self, err: Box<dyn Error>) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.read_error(err);
        } else {
            warn!("read_error reached end of pipeline");
        }
    }

    fn fire_transport_active(&mut self) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.transport_active();
        }
    }

    fn fire_transport_inactive(&mut self) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.transport_inactive();
        }
    }

    fn get_pipeline(&self) {} //TODO:
}

/* TODO:
impl<Rin, Rout, Context, H, Link> InboundLink<Rin>
    for InboundContextImpl<Rin, Rout, Context, H, Link>
where
    Context: InboundHandlerContext<Rout>,
    H: InboundHandler<Rin, Rout, Context>,
    Link: InboundLink<Rout>,
{
    fn read(&mut self, msg: Rin) {
        self.handler.read(self, msg);
    }

    fn read_eof(&mut self) {
        self.handler.read_eof(self);
    }

    fn read_error(&mut self, err: Box<dyn Error>) {
        self.handler.read_error(self, err);
    }

    fn transport_active(&mut self) {
        self.handler.transport_active(self);
    }

    fn transport_inactive(&mut self) {
        self.handler.transport_inactive(self);
    }
}*/

pub struct OutboundContextImpl<Win, Wout, Context, H, Link>
where
    Context: OutboundHandlerContext<Wout>,
    H: OutboundHandler<Win, Wout, Context>,
    Link: OutboundLink<Wout>,
{
    ctx: Option<Context>,
    pipeline: Weak<PipelineBase>,
    handler: H,
    next_out: Option<Link>,
    attached: bool,
    phantom_win: PhantomData<Win>,
    phantom_wout: PhantomData<Wout>,
}

impl<Win, Wout, Context, H, Link> OutboundContextImpl<Win, Wout, Context, H, Link>
where
    Context: OutboundHandlerContext<Wout>,
    H: OutboundHandler<Win, Wout, Context>,
    Link: OutboundLink<Wout>,
{
    pub fn get_handler(&self) -> &H {
        &self.handler
    }

    pub fn initialize(&mut self, pipeline: Weak<PipelineBase>, handler: H) {
        self.pipeline = pipeline;
        self.handler = handler;
    }
}

impl<Win, Wout, Context, H, Link> PipelineContext
    for OutboundContextImpl<Win, Wout, Context, H, Link>
where
    Context: OutboundHandlerContext<Wout>,
    H: OutboundHandler<Win, Wout, Context>,
    Link: OutboundLink<Wout>,
{
    fn get_direction(&self) -> HandlerDir {
        self.handler.get_direction()
    }

    fn attach_pipeline(&mut self) {
        if !self.attached {
            self.handler.attach_context(self.ctx.as_ref());
            self.attached = true;
        }
    }

    fn detach_pipeline(&mut self) {
        self.attached = false;
        self.handler.detach_context();
    }

    fn set_next_out(&mut self, _ctx: Option<Self>) {
        //TODO:
    }
}

impl<Win, Wout, Context, H, Link> OutboundHandlerContext<Wout>
    for OutboundContextImpl<Win, Wout, Context, H, Link>
where
    Context: OutboundHandlerContext<Wout>,
    H: OutboundHandler<Win, Wout, Context>,
    Link: OutboundLink<Wout>,
{
    fn fire_write(&mut self, msg: Wout) {
        if let Some(next_out) = self.next_out.as_mut() {
            next_out.write(msg);
        } else {
            warn!("write reached end of pipeline");
        }
    }

    fn fire_write_error(&mut self, err: Box<dyn Error>) {
        if let Some(next_out) = self.next_out.as_mut() {
            next_out.write_error(err);
        } else {
            warn!("write_error reached end of pipeline");
        }
    }

    fn fire_close(&mut self) {
        if let Some(next_out) = self.next_out.as_mut() {
            next_out.close();
        } else {
            warn!("close reached end of pipeline");
        }
    }

    fn get_pipeline(&self) {} //TODO:
}

pub struct ContextImpl<Rin, Rout, Win, Wout, Context, H, Inlink, Outlink>
where
    Context: HandlerContext<Rout, Wout>,
    H: Handler<Rin, Rout, Win, Wout, Context>,
    Inlink: InboundLink<Rout>,
    Outlink: OutboundLink<Wout>,
{
    ctx: Option<Context>,
    pipeline: Weak<PipelineBase>,
    handler: H,
    next_in: Option<Inlink>,
    next_out: Option<Outlink>,
    attached: bool,
    phantom_rin: PhantomData<Rin>,
    phantom_rout: PhantomData<Rout>,
    phantom_win: PhantomData<Win>,
    phantom_wout: PhantomData<Wout>,
}

impl<Rin, Rout, Win, Wout, Context, H, Inlink, Outlink>
    ContextImpl<Rin, Rout, Win, Wout, Context, H, Inlink, Outlink>
where
    Context: HandlerContext<Rout, Wout>,
    H: Handler<Rin, Rout, Win, Wout, Context>,
    Inlink: InboundLink<Rout>,
    Outlink: OutboundLink<Wout>,
{
    pub fn get_handler(&self) -> &H {
        &self.handler
    }

    pub fn initialize(&mut self, pipeline: Weak<PipelineBase>, handler: H) {
        self.pipeline = pipeline;
        self.handler = handler;
    }
}

impl<Rin, Rout, Win, Wout, Context, H, Inlink, Outlink> PipelineContext
    for ContextImpl<Rin, Rout, Win, Wout, Context, H, Inlink, Outlink>
where
    Context: HandlerContext<Rout, Wout>,
    H: Handler<Rin, Rout, Win, Wout, Context>,
    Inlink: InboundLink<Rout>,
    Outlink: OutboundLink<Wout>,
{
    fn get_direction(&self) -> HandlerDir {
        self.handler.get_direction()
    }

    fn attach_pipeline(&mut self) {
        if !self.attached {
            self.handler.attach_context(self.ctx.as_ref());
            self.attached = true;
        }
    }

    fn detach_pipeline(&mut self) {
        self.attached = false;
        self.handler.detach_context();
    }

    fn set_next_in(&mut self, ctx: Option<Self>) {
        //TODO:
        if ctx.is_none() {
            self.ctx = None;
            return;
        }
    }

    fn set_next_out(&mut self, _ctx: Option<Self>) {
        //TODO:
    }
}

impl<Rin, Rout, Win, Wout, Context, H, Inlink, Outlink> HandlerContext<Rout, Wout>
    for ContextImpl<Rin, Rout, Win, Wout, Context, H, Inlink, Outlink>
where
    Context: HandlerContext<Rout, Wout>,
    H: Handler<Rin, Rout, Win, Wout, Context>,
    Inlink: InboundLink<Rout>,
    Outlink: OutboundLink<Wout>,
{
    fn fire_read(&mut self, msg: Rout) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.read(msg);
        } else {
            warn!("read reached end of pipeline");
        }
    }

    fn fire_read_eof(&mut self) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.read_eof();
        } else {
            warn!("read_eof reached end of pipeline");
        }
    }

    fn fire_read_error(&mut self, err: Box<dyn Error>) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.read_error(err);
        } else {
            warn!("read_error reached end of pipeline");
        }
    }

    fn fire_transport_active(&mut self) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.transport_active();
        }
    }

    fn fire_transport_inactive(&mut self) {
        if let Some(next_in) = self.next_in.as_mut() {
            next_in.transport_inactive();
        }
    }

    fn fire_write(&mut self, msg: Wout) {
        if let Some(next_out) = self.next_out.as_mut() {
            next_out.write(msg);
        } else {
            warn!("write reached end of pipeline");
        }
    }

    fn fire_write_error(&mut self, err: Box<dyn Error>) {
        if let Some(next_out) = self.next_out.as_mut() {
            next_out.write_error(err);
        } else {
            warn!("write_error reached end of pipeline");
        }
    }

    fn fire_close(&mut self) {
        if let Some(next_out) = self.next_out.as_mut() {
            next_out.close();
        } else {
            warn!("close reached end of pipeline");
        }
    }

    fn get_pipeline(&self) {} //TODO:
}
