#![no_main]

use libfuzzer_sys::fuzz_target;

use eszip::Eszip;

fuzz_target!(|data: Vec<u8>| {
  let _ = futures::executor::block_on(fuzz(data));
});

async fn fuzz(data: Vec<u8>) -> Result<(), ()> {
  let data = std::io::Cursor::new(data);

  let (header, body) = Eszip::parse(data).await.map_err(|_| ())?;
  let _ = header.get_module("x");
  let body = body.await.map_err(|_| ())?;
  let _ = body.buffer();

  Ok(())
}
