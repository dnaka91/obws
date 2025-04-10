# Mapping from obs-websocket to obws

The following is a mapping from commands in the [`obs-websocket` docs] to the function calls in
`obws`. The latest API **v5** made significant changes to the naming style, and `obws` derives from
that.

This mapping may help developers, that come from the original obs-websocket documentation, to find
the equivalent function calls.

[`obs-websocket` docs]: https://github.com/obsproject/obs-websocket/blob/master/docs/generated/protocol.md

## General Requests

| obs-websocket              | obws                                                                                |
| -------------------------- | ----------------------------------------------------------------------------------- |
| GetVersion                 | [`General::get_version`](crate::client::General::version)                           |
| GetStats                   | [`General::stats`](crate::client::General::stats)                                   |
| BroadcastCustomEvent       | [`General::broadcast_custom_event`](crate::client::General::broadcast_custom_event) |
| CallVendorRequest          | [`General::call_vendor_request`](crate::client::General::call_vendor_request)       |
| GetHotkeyList              | [`Hotkeys::list`](crate::client::Hotkeys::list)                                     |
| TriggerHotkeyByName        | [`Hotkeys::trigger_by_name`](crate::client::Hotkeys::trigger_by_name)               |
| TriggerHotkeyByKeySequence | [`Hotkeys::trigger_by_sequence`](crate::client::Hotkeys::trigger_by_sequence)       |
| Sleep[^1]                  | -                                                                                   |

[^1]: Not implemented, as this command is only relevant for batch requests, which are not
      supported in `obws` yet.

## Config Requests

| obs-websocket             | obws                                                                                        |
| ------------------------- | ------------------------------------------------------------------------------------------- |
| GetPersistentData         | [`Config::get_persistent_data`](crate::client::Config::get_persistent_data)                 |
| SetPersistentData         | [`Config::set_persistent_data`](crate::client::Config::set_persistent_data)                 |
| GetSceneCollectionList    | [`SceneCollections::list`](crate::client::SceneCollections::list)                           |
| -                         | [`SceneCollections::current`](crate::client::SceneCollections::current)                     |
| SetCurrentSceneCollection | [`SceneCollections::set_current`](crate::client::SceneCollections::set_current)             |
| CreateSceneCollection     | [`SceneCollections::create`](crate::client::SceneCollections::create)                       |
| GetProfileList            | [`Profiles::list`](crate::client::Profiles::list)                                           |
| -                         | [`Profiles::current`](crate::client::Profiles::current)                                     |
| SetCurrentProfile         | [`Profiles::set_current`](crate::client::Profiles::set_current)                             |
| CreateProfile             | [`Profiles::create`](crate::client::Profiles::create)                                       |
| RemoveProfile             | [`Profiles::remove`](crate::client::Profiles::remove)                                       |
| GetProfileParameter       | [`Profiles::parameter`](crate::client::Profiles::parameter)                                 |
| SetProfileParameter       | [`Profiles::set_parameter`](crate::client::Profiles::set_parameter)                         |
| GetVideoSettings          | [`Config::video_settings`](crate::client::Config::video_settings)                           |
| SetVideoSettings          | [`Config::set_video_settings`](crate::client::Config::set_video_settings)                   |
| GetStreamServiceSettings  | [`Config::stream_service_settings`](crate::client::Config::stream_service_settings)         |
| SetStreamServiceSettings  | [`Config::set_stream_service_settings`](crate::client::Config::set_stream_service_settings) |
| GetRecordDirectory        | [`Config::record_directory`](crate::client::Config::record_directory)                       |

## Sources Requests

| obs-websocket        | obws                                                                  |
| -------------------- | --------------------------------------------------------------------- |
| GetSourceActive      | [`Sources::active`](crate::client::Sources::active)                   |
| GetSourceScreenshot  | [`Sources::take_screenshot`](crate::client::Sources::take_screenshot) |
| SaveSourceScreenshot | [`Sources::save_screenshot`](crate::client::Sources::save_screenshot) |

## Scenes Requests

| obs-websocket                   | obws                                                                                    |
| ------------------------------- | --------------------------------------------------------------------------------------- |
| GetSceneList                    | [`Scenes::list`](crate::client::Scenes::list)                                           |
| GetGroupList                    | [`Scenes::list_groups`](crate::client::Scenes::list_groups)                             |
| GetCurrentProgramScene          | [`Scenes::current_program_scene`](crate::client::Scenes::current_program_scene)         |
| SetCurrentProgramScene          | [`Scenes::set_current_program_scene`](crate::client::Scenes::set_current_program_scene) |
| GetCurrentPreviewScene          | [`Scenes::current_preview_scene`](crate::client::Scenes::current_preview_scene)         |
| SetCurrentPreviewScene          | [`Scenes::set_current_preview_scene`](crate::client::Scenes::set_current_preview_scene) |
| CreateScene                     | [`Scenes::create`](crate::client::Scenes::create)                                       |
| RemoveScene                     | [`Scenes::remove`](crate::client::Scenes::remove)                                       |
| SetSceneName                    | [`Scenes::set_name`](crate::client::Scenes::set_name)                                   |
| GetSceneSceneTransitionOverride | [`Scenes::transition_override`](crate::client::Scenes::transition_override)             |
| SetSceneSceneTransitionOverride | [`Scenes::set_transition_override`](crate::client::Scenes::set_transition_override)     |

## Inputs Requests

| obs-websocket                       | obws                                                                                              |
| ----------------------------------- | ------------------------------------------------------------------------------------------------- |
| GetInputList                        | [`Inputs::list`](crate::client::Inputs::list)                                                     |
| GetInputKindList                    | [`Inputs::list_kinds`](crate::client::Inputs::list_kinds)                                         |
| GetSpecialInputs                    | [`Inputs::specials`](crate::client::Inputs::specials)                                             |
| CreateInput                         | [`Inputs::create`](crate::client::Inputs::create)                                                 |
| RemoveInput                         | [`Inputs::remove`](crate::client::Inputs::remove)                                                 |
| SetInputName                        | [`Inputs::set_name`](crate::client::Inputs::set_name)                                             |
| GetInputDefaultSettings             | [`Inputs::default_settings`](crate::client::Inputs::default_settings)                             |
| GetInputSettings                    | [`Inputs::settings`](crate::client::Inputs::settings)                                             |
| SetInputSettings                    | [`Inputs::set_settings`](crate::client::Inputs::set_settings)                                     |
| GetInputMute                        | [`Inputs::muted`](crate::client::Inputs::muted)                                                   |
| SetInputMute                        | [`Inputs::set_muted`](crate::client::Inputs::set_muted)                                           |
| ToggleInputMute                     | [`Inputs::toggle_mute`](crate::client::Inputs::toggle_mute)                                       |
| GetInputVolume                      | [`Inputs::volume`](crate::client::Inputs::volume)                                                 |
| SetInputVolume                      | [`Inputs::set_volume`](crate::client::Inputs::set_volume)                                         |
| GetInputAudioBalance                | [`Inputs::audio_balance`](crate::client::Inputs::audio_balance)                                    |
| SetInputAudioBalance                | [`Inputs::set_audio_balance`](crate::client::Inputs::set_audio_balance)                            |
| GetInputAudioSyncOffset             | [`Inputs::audio_sync_offset`](crate::client::Inputs::audio_sync_offset)                           |
| SetInputAudioSyncOffset             | [`Inputs::set_audio_sync_offset`](crate::client::Inputs::set_audio_sync_offset)                   |
| GetInputAudioMonitorType            | [`Inputs::audio_monitor_type`](crate::client::Inputs::audio_monitor_type)                         |
| SetInputAudioMonitorType            | [`Inputs::set_audio_monitor_type`](crate::client::Inputs::set_audio_monitor_type)                 |
| GetInputAudioTracks                 | [`Inputs::audio_tracks`](crate::client::Inputs::audio_tracks)                                      |
| SetInputAudioTracks                 | [`Inputs::set_audio_tracks`](crate::client::Inputs::set_audio_tracks)                              |
| GetInputPropertiesListPropertyItems | [`Inputs::properties_list_property_items`](crate::client::Inputs::properties_list_property_items) |
| PressInputPropertiesButton          | [`Inputs::press_properties_button`](crate::client::Inputs::press_properties_button)               |

## Transitions Requests

| obs-websocket                     | obws                                                                                    |
| --------------------------------- | --------------------------------------------------------------------------------------- |
| GetTransitionKindList             | [`Transitions::list_kinds`](crate::client::Transitions::list_kinds)                     |
| GetSceneTransitionList            | [`Transitions::list`](crate::client::Transitions::list)                                 |
| GetCurrentSceneTransition         | [`Transitions::current`](crate::client::Transitions::current)                           |
| SetCurrentSceneTransition         | [`Transitions::set_current`](crate::client::Transitions::set_current)                   |
| SetCurrentSceneTransitionDuration | [`Transitions::set_current_duration`](crate::client::Transitions::set_current_duration) |
| SetCurrentSceneTransitionSettings | [`Transitions::set_current_settings`](crate::client::Transitions::set_current_settings) |
| GetCurrentSceneTransitionCursor   | [`Transitions::current_cursor`](crate::client::Transitions::current_cursor)             |
| TriggerStudioModeTransition       | [`Transitions::trigger`](crate::client::Transitions::trigger)                           |
| SetTBarPosition                   | [`Transitions::set_tbar_position`](crate::client::Transitions::set_tbar_position)       |

## Filters Requests

| obs-websocket                  | obws                                                                    |
| ------------------------------ | ----------------------------------------------------------------------- |
| GetSourceFilterList            | [`Filters::list`](crate::client::Filters::list)                         |
| GetSourceFilterDefaultSettings | [`Filters::default_settings`](crate::client::Filters::default_settings) |
| CreateSourceFilter             | [`Filters::create`](crate::client::Filters::create)                     |
| RemoveSourceFilter             | [`Filters::remove`](crate::client::Filters::remove)                     |
| SetSourceFilterName            | [`Filters::set_name`](crate::client::Filters::set_name)                 |
| GetSourceFilter                | [`Filters::get`](crate::client::Filters::get)                           |
| SetSourceFilterIndex           | [`Filters::set_index`](crate::client::Filters::set_index)               |
| SetSourceFilterSettings        | [`Filters::set_settings`](crate::client::Filters::set_settings)         |
| SetSourceFilterEnabled         | [`Filters::set_enabled`](crate::client::Filters::set_enabled)           |

## Scene Items Requests

| obs-websocket                   | obws                                                                                  |
| ------------------------------- | ------------------------------------------------------------------------------------- |
| GetSceneItemList                | [`SceneItems::list`](crate::client::SceneItems::list)                                 |
| GetGroupSceneItemList           | [`SceneItems::list_group`](crate::client::SceneItems::list_group)                     |
| GetSceneItemId                  | [`SceneItems::id`](crate::client::SceneItems::id)                                     |
| CreateSceneItem                 | [`SceneItems::create`](crate::client::SceneItems::create)                             |
| RemoveSceneItem                 | [`SceneItems::remove`](crate::client::SceneItems::remove)                             |
| DuplicateSceneItem              | [`SceneItems::duplicate`](crate::client::SceneItems::duplicate)                       |
| GetSceneItemTransform           | [`SceneItems::transform`](crate::client::SceneItems::transform)                       |
| SetSceneItemTransform           | [`SceneItems::set_transform`](crate::client::SceneItems::set_transform)               |
| GetSceneItemEnabled             | [`SceneItems::enabled`](crate::client::SceneItems::enabled)                           |
| SetSceneItemEnabled             | [`SceneItems::set_enabled`](crate::client::SceneItems::set_enabled)                   |
| GetSceneItemLocked              | [`SceneItems::locked`](crate::client::SceneItems::locked)                             |
| SetSceneItemLocked              | [`SceneItems::set_locked`](crate::client::SceneItems::set_locked)                     |
| GetSceneItemIndex               | [`SceneItems::index`](crate::client::SceneItems::index)                               |
| SetSceneItemIndex               | [`SceneItems::set_index`](crate::client::SceneItems::set_index)                       |
| GetSceneItemBlendMode           | [`SceneItems::blend_mode`](crate::client::SceneItems::blend_mode)                     |
| SetSceneItemBlendMode           | [`SceneItems::set_blend_mode`](crate::client::SceneItems::set_blend_mode)             |
| GetSceneItemPrivateSettings[^2] | [`SceneItems::private_settings`](crate::client::SceneItems::private_settings)         |
| SetSceneItemPrivateSettings[^2] | [`SceneItems::set_private_settings`](crate::client::SceneItems::set_private_settings) |

[^2]: Hidden in the official obs-websocket docs, as these are _dangerous_ commands.

## Outputs Requests

| obs-websocket             | obws                                                                    |
| ------------------------- | ----------------------------------------------------------------------- |
| GetVirtualCamStatus       | [`VirtualCam::status`](crate::client::VirtualCam::status)               |
| ToggleVirtualCam          | [`VirtualCam::toggle`](crate::client::VirtualCam::toggle)               |
| StartVirtualCam           | [`VirtualCam::start`](crate::client::VirtualCam::start)                 |
| StopVirtualCam            | [`VirtualCam::stop`](crate::client::VirtualCam::stop)                   |
| GetReplayBufferStatus     | [`ReplayBuffer::status`](crate::client::ReplayBuffer::status)           |
| ToggleReplayBuffer        | [`ReplayBuffer::toggle`](crate::client::ReplayBuffer::toggle)           |
| StartReplayBuffer         | [`ReplayBuffer::start`](crate::client::ReplayBuffer::start)             |
| StopReplayBuffer          | [`ReplayBuffer::stop`](crate::client::ReplayBuffer::stop)               |
| SaveReplayBuffer          | [`ReplayBuffer::save`](crate::client::ReplayBuffer::save)               |
| GetLastReplayBufferReplay | [`ReplayBuffer::last_replay`](crate::client::ReplayBuffer::last_replay) |
| GetOutputList             | [`Outputs::list`](crate::client::Outputs::list)                         |
| GetOutputStatus           | [`Outputs::status`](crate::client::Outputs::status)                     |
| ToggleOutput              | [`Outputs::toggle`](crate::client::Outputs::toggle)                     |
| StartOutput               | [`Outputs::start`](crate::client::Outputs::start)                       |
| StopOutput                | [`Outputs::stop`](crate::client::Outputs::stop)                         |
| GetOutputSettings         | [`Outputs::settings`](crate::client::Outputs::settings)                 |
| SetOutputSettings         | [`Outputs::set_settings`](crate::client::Outputs::set_settings)         |

## Stream Requests

| obs-websocket     | obws                                                                |
| ----------------- | ------------------------------------------------------------------- |
| GetStreamStatus   | [`Streaming::status`](crate::client::Streaming::status)             |
| ToggleStream      | [`Streaming::toggle`](crate::client::Streaming::toggle)             |
| StartStream       | [`Streaming::start`](crate::client::Streaming::start)               |
| StopStream        | [`Streaming::stop`](crate::client::Streaming::stop)                 |
| SendStreamCaption | [`Streaming::send_caption`](crate::client::Streaming::send_caption) |

## Record Requests

| obs-websocket       | obws                                                                    |
| ------------------- | ----------------------------------------------------------------------- |
| GetRecordStatus     | [`Recording::status`](crate::client::Recording::status)                 |
| ToggleRecord        | [`Recording::toggle`](crate::client::Recording::toggle)                 |
| StartRecord         | [`Recording::start`](crate::client::Recording::start)                   |
| StopRecord          | [`Recording::stop`](crate::client::Recording::stop)                     |
| ToggleRecordPause   | [`Recording::toggle_pause`](crate::client::Recording::toggle_pause)     |
| PauseRecord         | [`Recording::pause`](crate::client::Recording::pause)                   |
| ResumeRecord        | [`Recording::resume`](crate::client::Recording::resume)                 |
| SplitRecordFile     | [`Recording::split_file`](crate::client::Recording::split_file)         |
| CreateRecordChapter | [`Recording::create_chapter`](crate::client::Recording::create_chapter) |

## Media Inputs Requests

| obs-websocket           | obws                                                                        |
| ----------------------- | --------------------------------------------------------------------------- |
| GetMediaInputStatus     | [`MediaInputs::status`](crate::client::MediaInputs::status)                 |
| SetMediaInputCursor     | [`MediaInputs::set_cursor`](crate::client::MediaInputs::set_cursor)         |
| OffsetMediaInputCursor  | [`MediaInputs::offset_cursor`](crate::client::MediaInputs::offset_cursor)   |
| TriggerMediaInputAction | [`MediaInputs::trigger_action`](crate::client::MediaInputs::trigger_action) |

## UI Requests

| obs-websocket             | obws                                                                          |
| ------------------------- | ----------------------------------------------------------------------------- |
| GetStudioModeEnabled      | [`Ui::studio_mode_enabled`](crate::client::Ui::studio_mode_enabled)           |
| SetStudioModeEnabled      | [`Ui::set_studio_mode_enabled`](crate::client::Ui::set_studio_mode_enabled)   |
| OpenInputPropertiesDialog | [`Ui::open_properties_dialog`](crate::client::Ui::open_properties_dialog)     |
| OpenInputFiltersDialog    | [`Ui::open_filters_dialog`](crate::client::Ui::open_filters_dialog)           |
| OpenInputInteractDialog   | [`Ui::open_interact_dialog`](crate::client::Ui::open_interact_dialog)         |
| GetMonitorList            | [`Ui::list_monitors`](crate::client::Ui::list_monitors)                       |
| OpenVideoMixProjector     | [`Ui::open_video_mix_projector`](crate::client::Ui::open_video_mix_projector) |
| OpenSourceProjector       | [`Ui::open_source_projector`](crate::client::Ui::open_source_projector)       |
