# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

load("@bazel_skylib//lib:new_sets.bzl", "sets")
load("@bazel_skylib//lib:paths.bzl", "paths")

glob = native.glob

def subdir_glob(glob_specs, exclude = None, prefix = ""):
    """Returns a dict of sub-directory relative paths to full paths.

    The subdir_glob() function is useful for defining header maps for C/C++
    libraries which should be relative the given sub-directory.
    Given a list of tuples, the form of (relative-sub-directory, glob-pattern),
    it returns a dict of sub-directory relative paths to full paths.

    Please refer to native.glob() for explanations and examples of the pattern.

    Args:
      glob_specs: The array of tuples in form of
        (relative-sub-directory, glob-pattern inside relative-sub-directory).
        type: List[Tuple[str, str]]
      exclude: A list of patterns to identify files that should be removed
        from the set specified by the first argument. Defaults to [].
        type: Optional[List[str]]
      prefix: If is not None, prepends it to each key in the dictionary.
        Defaults to None.
        type: Optional[str]

    Returns:
      A dict of sub-directory relative paths to full paths.
    """
    if exclude == None:
        exclude = []

    results = []

    for dirpath, glob_pattern in glob_specs:
        results.append(
            _single_subdir_glob(dirpath, glob_pattern, exclude, prefix),
        )

    return _merge_maps(*results)

def _merge_maps(*file_maps):
    result = {}
    for file_map in file_maps:
        for key in file_map:
            if key in result and result[key] != file_map[key]:
                fail(
                    "Conflicting files in file search paths. " +
                    "\"%s\" maps to both \"%s\" and \"%s\"." %
                    (key, result[key], file_map[key]),
                )

            result[key] = file_map[key]

    return result

def _single_subdir_glob(dirpath, glob_pattern, exclude = None, prefix = None, strict = False):
    if exclude == None:
        exclude = []
    results = {}
    glob_func = (lambda *args, **kwargs: strict_glob(_called_by_subdir_glob = True, *args, **kwargs)) if strict else native.glob
    files = glob_func([paths.join(dirpath, glob_pattern)], exclude = exclude)
    for f in files:
        if dirpath:
            key = f[len(dirpath) + 1:]
        else:
            key = f
        if prefix:
            key = paths.join(prefix, key)
        results[key] = f

    return results

def strict_glob(include, _called_by_subdir_glob = False, **kwargs):
    """Has the same API as glob(), but will fail() if any entry in include fails to match."""
    has_glob_pattern = False
    all_results = []
    for entry in include:
        has_glob_pattern = has_glob_pattern or "*" in entry

        result = native.glob([entry], **kwargs)
        if len(result) == 0:
            fail("Strict glob failed to match files for glob pattern: " + entry)
        all_results.extend(result)

    # Dedupe.
    if len(include) > 1:
        all_results = sets.to_list(sets.make(all_results))

    for entry in kwargs.get("exclude", []):
        has_glob_pattern = has_glob_pattern or "*" in entry

    # When called by subdir_glob, we pass each entry individually, meaning we won't observe the
    # other entries with "*" when processing a single entry that may not have it.
    if not _called_by_subdir_glob and not has_glob_pattern:
        fail("Using a strict glob without a single '*'. Use a literal list instead of globbing.")

    return all_results
