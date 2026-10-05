# Legacy development source

This directory preserves superseded Complete Story implementations exactly for debugging and historical review.

- `CompleteStory13thTest` is the abandoned early F8/F9 hotkey prototype. It is not part of the current design and must not be described as current functionality.
- `v0.6-runtime` is the unsafe UE4SS experiment that used global UObject and text-widget searches inside callbacks. It is retained to compare the failure against later implementations.

Neither directory is a working release. The active failing experiment is `runtime/v0.7`.
