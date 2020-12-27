# Examples

These are several examples that show how to use `obws`. If you require authentication for your OBS
instance, create a `.env` file in the project root and add a `OBS_PASSWORD` entry in it. The
examples will pick up the password and authenticate automatically.

- `simple` A very basic example showing how to connect, login and print out current version
  information and the list of scenes.
- `iter_scenes` Get a list of all scenes and endlessly iterate through them with a small pause
  between each change.
- `screenshot` Take a screenshot of the currently visible scene and save it as `screenshot.png`.
- `events` Shows how to get a stream of user events and simply print them out to the terminal.
