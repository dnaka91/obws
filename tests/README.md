# Integration tests

To run integration tests, obws will connect to your OBS instance and send several commands against
the obs-websocket API to make sure most of the API works as expected.

For this to work, a few settings need to be set and some scene items created so that the tests have
items to work on. This has to be done manually as the API doesn't allow to create new sources and
scenes or modify specific settings.

- Use at least OBS version `27.0.0`.
- Create a **source collection** called `OBWS-TEST`.
- Create a **profile** called `OBWS-TEST`.
- Create two **scene**s called `OBWS-TEST-Scene` and `OBWS-TEST-Scene2`.
- Create two **Freetype2 text source**s called `OBWS-TEST-Text` and `OBWS-TEST-Text2`.
- Create a **browser source** called `OBWS-TEST-Browser`.
- Create a **VLC media source** called `OBWS-TEST-Media` and add a folder with videos to the
  playlist.
- Create two **transition**s called `OBWS-TEST-Transition` and `OBWS-TEST-Transition2`.
- Make sure a global **Desktop Audio** device is configured.
- Set any **hotkey** to `P` without any modifier keys (like _ctrl_ or _alt_).
