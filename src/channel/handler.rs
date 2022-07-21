use super::handler_context::*;

use std::error::Error;

pub enum HandlerDir {
    IN,
    OUT,
    BOTH,
}

pub trait HandlerBase<Context> {
    fn get_context(&self) -> Option<&Context>;
    fn set_context(&mut self, ctx: Option<&Context>);
    fn get_attach_count(&self) -> usize;
    fn set_attach_count(&mut self, _count: usize);

    fn attach_context(&mut self, ctx: Option<&Context>) {
        let count = self.get_attach_count() + 1;
        self.set_attach_count(count);
        if count == 1 {
            self.set_context(ctx);
        } else {
            self.set_context(None);
        }
    }

    fn detach_context(&mut self) {
        let count = self.get_attach_count();
        if count >= 1 {
            self.set_attach_count(count - 1);
        }
        self.set_context(None);
    }
}

pub trait InboundHandler<Rin, Rout, Context: InboundHandlerContext<Rout>>:
    HandlerBase<Context>
{
    fn get_direction(&self) -> HandlerDir {
        HandlerDir::IN
    }

    fn read(&mut self, ctx: &mut Context, msg: Rin);

    fn read_eof(&mut self, ctx: &mut Context) {
        ctx.fire_read_eof();
    }

    fn read_error(&mut self, ctx: &mut Context, err: Box<dyn Error>) {
        ctx.fire_read_error(err);
    }

    fn transport_active(&mut self, ctx: &mut Context) {
        ctx.fire_transport_active();
    }

    fn transport_inactive(&mut self, ctx: &mut Context) {
        ctx.fire_transport_inactive();
    }
}

pub trait OutboundHandler<Win, Wout, Context: OutboundHandlerContext<Wout>>:
    HandlerBase<Context>
{
    fn get_direction(&self) -> HandlerDir {
        HandlerDir::OUT
    }

    fn write(&mut self, ctx: &mut Context, msg: Win);

    fn write_error(&mut self, ctx: &mut Context, err: Box<dyn Error>) {
        ctx.fire_write_error(err);
    }

    fn close(&mut self, ctx: &mut Context) {
        ctx.fire_close();
    }
}

pub trait Handler<Rin, Rout, Win, Wout, Context: HandlerContext<Rout, Wout>>:
    HandlerBase<Context>
{
    fn get_direction(&self) -> HandlerDir {
        HandlerDir::BOTH
    }

    fn read(&mut self, ctx: &mut Context, msg: Rin);

    fn read_eof(&mut self, ctx: &mut Context) {
        ctx.fire_read_eof();
    }

    fn read_error(&mut self, ctx: &mut Context, err: Box<dyn Error>) {
        ctx.fire_read_error(err);
    }

    fn transport_active(&mut self, ctx: &mut Context) {
        ctx.fire_transport_active();
    }

    fn transport_inactive(&mut self, ctx: &mut Context) {
        ctx.fire_transport_inactive();
    }

    fn write(&mut self, ctx: &mut Context, msg: Win);

    fn write_error(&mut self, ctx: &mut Context, err: Box<dyn Error>) {
        ctx.fire_write_error(err);
    }

    fn close(&mut self, ctx: &mut Context) {
        ctx.fire_close();
    }
}

pub trait HandlerAdapter<R, W, Context: HandlerContext<R, W>>:
    Handler<R, R, W, W, Context>
{
    fn read(&mut self, ctx: &mut Context, msg: R) {
        ctx.fire_read(msg);
    }

    fn write(&mut self, ctx: &mut Context, msg: W) {
        ctx.fire_write(msg);
    }
}
