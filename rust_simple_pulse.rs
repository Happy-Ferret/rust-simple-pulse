use std::libc::{c_int, c_char, size_t};
use std::ptr;
use std::io::File;
use std::path::Path;
use std::os;

#[link_args = "-lpulse-simple -lpulse"]
extern {
  fn pa_simple_new(server: *c_char, 
                   name: *c_char,
                   dir: c_int,
                   dev: *c_char,
                   steam_name: *c_char,
                   sample_spec: *pa_sample_spec,
                   channel_map: *u8,
                   attr: *u8,
                   error: *mut c_int) -> *mut pa_simple;

  fn pa_simple_free(pa: *mut pa_simple);

  fn pa_simple_write(pa: *mut pa_simple, 
                     data: *u8,
                     bytes: size_t,
                     error: *mut c_int) -> c_int;

  fn pa_simple_drain(pa: *mut pa_simple,
                     error: *mut c_int) -> c_int;

  fn pa_strerror(error: c_int) -> *c_char;
}

// typedef struct pa_simple pa_simple 
pub struct pa_simple;

// defined as enum pa_stream_direction
//pub static PA_STREAM_NODIRECTION: c_int = 0_i32;
pub static PA_STREAM_PLAYBACK:    c_int = 1_i32;
//pub static PA_STREAM_RECORD:      c_int = 2_i32;
//pub static PA_STREAM_UPLOAD:      c_int = 3_i32; 

// see pa_sample_format
pub static PA_SAMPLE_S16LE: c_int = 3_i32;

// see pulse/def.h
pub struct pa_sample_spec {
  format: c_int,
  rate: u32,
  channels: u8
} 



// --------------------
pub fn pa_new(pa_name: &str, stream_name: &str) -> ~*mut pa_simple {
  unsafe {
    let mut err: c_int = 0;

    let s_spec = pa_sample_spec{
                      format: PA_SAMPLE_S16LE, 
                      rate: 44100, 
                      channels: 2};

    let pa = pa_simple_new( 
                  ptr::null(), 
                  pa_name.to_c_str().unwrap(),
                  PA_STREAM_PLAYBACK, 
                  ptr::null(), 
                  stream_name.to_c_str().unwrap(), 
                  ptr::to_unsafe_ptr(&s_spec),
                  ptr::null(),
                  ptr::null(),
                  &mut err);
    if ( err != 0 ) {
      fail!("err code {} from pulse: \"{}\"", 
               err, std::str::raw::from_c_str(pa_strerror(err)) );
    }
    ~pa // cast to region pointer, owning pointer
  }
}

//---------------------------------------------

pub fn play_file(pa: &*mut pa_simple, path: &Path) -> bool {
  if ( !path.is_file() ) {
    println("This is not a file!");
    return false;
  }

  println!("Gonna play: {}", path.as_str().unwrap());

  let mut err: c_int = 0;
  let mut file_reader = File::open(path);
  unsafe {
    static BUFSIZE: uint = 1024u; 
    let mut buffer: [u8,..BUFSIZE] = [0u8, ..BUFSIZE];
    let mut total_read = 0u;
    loop {
      let b_read = match file_reader.read(buffer) {
        None => break, // eof
             Some(s) => s//read smth
      };
      let w_res = pa_simple_write(
                    *pa,
                    std::vec::raw::to_ptr(buffer),
                    b_read as size_t,
                    &mut err);
      if ( w_res < 0) {
        println!("ERROR code {} from pulse: \"{}\"", 
                 err, std::str::raw::from_c_str(pa_strerror(err)) );
        return false;
      }
      total_read += b_read;
    }
    println!("bytes read: {}", total_read);

    pa_simple_drain(*pa, &mut err);
  }
  true
}

//---------------------------------------------

pub fn free_pa(pa: &*mut pa_simple) {
  unsafe {
    pa_simple_free(*pa);
  }
}

//---------------------------------------------

fn main()
{
  let args = os::args();
  if ( args.len() != 2 ) {
    fail!("BAAHH I need a file to play as a parameter.");
  }
  let f_name = args[1];

  let path = ~Path::new(f_name);
  let pa_name = "rust_simple_pulse";
  let stream_name  = "rust_playback";

  let pa = pa_new(pa_name, stream_name);

  if ( !play_file(pa, path) )
  {
    fail!("Dude I was not able to play the file.");
  }
  free_pa(pa);
}
