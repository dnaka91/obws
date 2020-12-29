# Integration tests

To run integration tests, obws will connect to your OBS instance and send several commands against
the obs-websocket API to make sure most of the API works as expected.

For this to work, a few settings need to be set and some scene items created so that the tests have
items to work on. This has to be done manually as the API doesn't allow to create new sources and
scenes or modify specific settings.

- Use at least OBS version `26.1.0`.
- Create a **source collection** called `OBWS-TEST`.
- Create a **scene** called `OBWS-TEST-Scene`.
- Create a **Freetype2 text source** called `OBWS-TEST-Text`.
- Make sure a global **Desktop Audio** device is configured.
