use std::error::Error;

pub trait InboundHandlerContext<In> {
    fn fire_read(&mut self, msg: In);
    fn fire_read_eof(&mut self);
    fn fire_read_error(&mut self, err: Box<dyn Error>);

    fn fire_transport_active(&mut self);
    fn fire_transport_inactive(&mut self);

    fn get_pipeline(&self); //TODO: -> PipelineBase
                            //TODO: fn get_pipeline_shared(&self) -> Arc<PipelineBase>;
                            //TODO: fn get_transport(&self) -> ?
}

pub trait OutboundHandlerContext<Out> {
    fn fire_write(&mut self, msg: Out);
    fn fire_write_error(&mut self, err: Box<dyn Error>);
    fn fire_close(&mut self);

    fn get_pipeline(&self); //TODO: -> PipelineBase
                            //TODO: fn get_pipeline_shared(&self) -> Arc<PipelineBase>;
                            //TODO: fn get_transport(&self) -> ?
}

pub trait HandlerContext<In, Out> {
    fn fire_read(&mut self, msg: In);
    fn fire_read_eof(&mut self);
    fn fire_read_error(&mut self, err: Box<dyn Error>);

    fn fire_transport_active(&mut self);
    fn fire_transport_inactive(&mut self);

    fn fire_write(&mut self, msg: Out);
    fn fire_write_error(&mut self, err: Box<dyn Error>);
    fn fire_close(&mut self);

    fn get_pipeline(&self); //TODO: -> PipelineBase
                            //TODO: fn get_pipeline_shared(&self) -> Arc<PipelineBase>;
                            //TODO: fn get_transport(&self) -> ?

    /*
     virtual void setWriteFlags(folly::WriteFlags flags) = 0;
     virtual folly::WriteFlags getWriteFlags() = 0;

     virtual void setReadBufferSettings(
         uint64_t minAvailable,
         uint64_t allocationSize) = 0;
     virtual std::pair<uint64_t, uint64_t> getReadBufferSettings() = 0;
    */
}
