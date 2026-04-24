use rodio::{Sink, OutputStream };
use rodio::source::{Source, SineWave};
use std::time::Duration;

pub struct Audio
{
    _stream : OutputStream,
    sink : Sink,
}

impl Audio
{
    // Constructor
    pub fn new() -> Self
    {
        let ( _stream, stream_handle ) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new( &stream_handle ).unwrap();
        
        sink.pause();
        Self { _stream, sink }
    }

    // play
    pub fn play( &self )
    {
        if self.sink.is_paused()
        {
            // new sine wave at 440 hz
            let source = SineWave::new( 440.0 )
                .take_duration( Duration::from_secs( 1 ))
                .amplify( 0.2 )
                .repeat_infinite();

            self.sink.append( source );
            self.sink.play()
        }
    }

    // Stop
    pub fn stop( &self )
    {
        if !self.sink.is_paused()
        {
            self.sink.pause();
            self.sink.clear();
        }
    }
}