
use mpegts::stream_id::*;

pub fn get_stream_id(stream_id: u8) -> StreamId {
  match stream_id {
    0x00 => StreamId::Reserved,
    0x01 => StreamId::IsoIec111722Mpeg1Video,
    0x02 => StreamId::IsoIec138182Mpeg2Video,
    0x03 => StreamId::IsoIec111723Mpeg1Audio,
    0x04 => StreamId::IsoIec138183Mpeg2Audio,
    0x05 => StreamId::IsoIec138181PrivateSection,
    0x06 => StreamId::IsoIec138181Pes,
    0x07 => StreamId::IsoIec13522Mheg,
    0x08 => StreamId::ItuTH2220AnnexADsmCc,
    0x09 => StreamId::ItuTH2221,
    0x0a => StreamId::IsoIec138186DsmCcTypeA,
    0x0b => StreamId::IsoIec138186DsmCcTypeB,
    0x0c => StreamId::IsoIec138186DsmCcTypeC,
    0x0d => StreamId::IsoIec138186DsmCcTypeD,
    0x0e => StreamId::IsoIec138181Auxiliary,
    0x0f => StreamId::IsoIec138187AacAudio,
    0x10 => StreamId::IsoIec144962Mpeg4Video,
    0x11 => StreamId::IsoIec144963AacLatmAudio,
    0x1b => StreamId::ItuTH264Video,
    0x24 => StreamId::ItuTH265Video,
    0xea => StreamId::Vc1Video,
    0xd1 => StreamId::DiracVideo,
    0x81 => StreamId::Ac3Audio,
    0x8a => StreamId::DtsAudio,
    0xbd => StreamId::NonMpegAudioSubpictures,
    0xbe => StreamId::PaddingStream,
    0xbf => StreamId::NavigationData,
       _ => {
      if (stream_id >= 0xc0) && (stream_id <= 0xdf) {
        StreamId::AudioStream{id: stream_id}
      } else {
        if (stream_id >= 0xe0) && (stream_id <= 0xef) {
          StreamId::VideoStream{id: stream_id}
        } else {
          println!("Unknown Stream ID {:?}", stream_id);
          StreamId::Unknown
        }
      }
    }
  }
}
