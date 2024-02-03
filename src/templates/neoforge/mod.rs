// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Build files
super::file_data!(BUILD_GRADLE build_gradle, "neoforge", true, "build.gradle");
super::file_data!(GRADLE_PROPERTIES gradle_properties, "neoforge", true, "gradle.properties");

// Code
super::file_data!(MODS_TOML mods_toml, "neoforge", true, "src/main/resources/META-INF/mods.toml");
super::file_data!(MOD_CLASS mod_class, "neoforge", true, "src/main/java/PACKAGE_DIR/neoforge/ExampleModNeoForge.java");

super::file_list!(pub all_files,
    build_gradle
    gradle_properties
    mods_toml
    mod_class
);
