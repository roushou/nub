use chrono::Datelike;
use clap::{Args, Subcommand, ValueEnum};

use crate::errors::CliError;

#[derive(Args)]
pub(crate) struct LicenseCommand {
    #[command(subcommand)]
    command: SubCommands,
}

impl LicenseCommand {
    pub fn run(&self) -> Result<(), CliError> {
        match &self.command {
            SubCommands::Use(cmd) => cmd.run(),
            SubCommands::List(cmd) => cmd.run(),
        }
    }
}

/// Availalbe subcommands for license operations.
#[derive(Subcommand)]
enum SubCommands {
    /// Use a license.
    Use(UseSubCommand),

    /// List available licenses
    List(ListSubCommand),
}

/// Arguments for the list subcommand.
#[derive(Args)]
struct ListSubCommand {
    #[arg(long, help = "Filter licenses by type")]
    permission: Option<LicensePermission>,
}

impl ListSubCommand {
    pub fn run(&self) -> Result<(), CliError> {
        let licenses = vec![
            (
                LicenseKind::Mit,
                "MIT",
                "Permissive",
                "A short, permissive license allowing almost unrestricted use with a copyright notice.",
            ),
            (
                LicenseKind::Apache,
                "Apache-2.0",
                "Permissive",
                "Permissive with patent grant, suitable for corporate use.",
            ),
            (
                LicenseKind::Gpl,
                "GPL-3.0",
                "Copyleft",
                "Strong copyleft; derivative works must be open-source.",
            ),
            (
                LicenseKind::Lgpl,
                "LGPL-3.0",
                "Copyleft",
                "Weaker copyleft for libraries; allows linking in proprietary software.",
            ),
            (
                LicenseKind::Mpl,
                "MPL-2.0",
                "Copyleft",
                "File-based copyleft; balances permissive and copyleft principles.",
            ),
            (
                LicenseKind::Bsd,
                "BSD-3-Clause",
                "Permissive",
                "Permissive with endorsement restrictions, similar to MIT.",
            ),
            (
                LicenseKind::Unlicense,
                "Unlicense",
                "Public Domain",
                "Dedicates software to the public domain, maximally permissive.",
            ),
        ];

        let mut filtered_licenses: Vec<_> = match &self.permission {
            Some(permission) => licenses
                .into_iter()
                .filter(|&(_, _, t, _)| t == permission.as_str())
                .collect(),
            None => licenses,
        };
        // Sort licenses by name for consistent output
        filtered_licenses.sort_by(|a, b| a.1.cmp(b.1));

        println!("Available Licenses:");
        println!("{:-<80}", "");
        println!("{:<20} {:<15} Description", "Name", "Type");
        println!("{:-<80}", "");

        for (kind, name, permission, description) in filtered_licenses {
            println!("{name:<20} {permission:<15} {description}");
            match kind {
                LicenseKind::Mit => {
                    println!("> Key Terms: Include copyright notice, no warranty.")
                }
                LicenseKind::Apache => println!(
                    "> Key Terms: Patent grant, explicit license terms at http://www.apache.org/licenses/LICENSE-2.0."
                ),
                LicenseKind::Gpl => println!(
                    "> Key Terms: Source code must be shared, see https://www.gnu.org/licenses/gpl-3.0.html."
                ),
                LicenseKind::Lgpl => println!(
                    "> Key Terms: Library use in proprietary software allowed, see https://www.gnu.org/licenses/lgpl-3.0.html."
                ),
                LicenseKind::Mpl => {
                    println!("> Key Terms: File-based copyleft, see https://mozilla.org/MPL/2.0/.")
                }
                LicenseKind::Bsd => {
                    println!("> Key Terms: No endorsement clause, include copyright notice.")
                }
                LicenseKind::Unlicense => println!(
                    "> Key Terms: Public domain, no restrictions, see http://unlicense.org/."
                ),
            }
            println!("{:-<80}", "");
        }

        Ok(())
    }
}

/// Arguments for the create subcommand.
#[derive(Args)]
struct UseSubCommand {
    #[arg(help = "The license type to use (e.g. mit, apache)")]
    kind: LicenseKind,

    #[arg(long, help = "The name or organization for the license")]
    name: Option<String>,

    #[arg(
        long,
        help = "The year for the copyright notice (defaults to current year)",
        default_value_t = chrono::Local::now().year()
    )]
    year: i32,
}

impl UseSubCommand {
    pub fn run(&self) -> Result<(), CliError> {
        let name = self
            .name
            .clone()
            .unwrap_or("<YOUR NAME/ORGANIZATION>".to_string());
        if name.trim().is_empty() {
            return Err(CliError::InvalidInput);
        }
        let license = self.kind.create(name, self.year);
        println!("{license}");
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LicenseKind {
    Mit,
    Apache,
    Gpl,
    Lgpl,
    Mpl,
    Bsd,
    Unlicense,
}

impl LicenseKind {
    pub fn create(&self, name: String, year: i32) -> String {
        match self {
            LicenseKind::Mit => format!(
                r#"
MIT License

Copyright (c) {year}-present {name}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
                "#
            ),
            LicenseKind::Apache => format!(
                r#"
Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

Copyright {year} {name}

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
"#
            ),
            LicenseKind::Gpl => format!(
                r#"
GNU General Public License
Version 3, 29 June 2007

Copyright (C) {year} {name}

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
            "#
            ),
            LicenseKind::Lgpl => format!(
                r#"
GNU Lesser General Public License
Version 3, 29 June 2007

Copyright (C) {year} {name}

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation; either
version 3 of the License, or (at your option) any later version.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public
License along with this library; if not, see <https://www.gnu.org/licenses/>.
            "#
            ),
            LicenseKind::Mpl => format!(
                r#"
Mozilla Public License
Version 2.0

Copyright (c) {year} {name}

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
            "#
            ),
            LicenseKind::Bsd => format!(
                r#"
BSD 3-Clause License

Copyright (c) {year} {name}

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
            "#
            ),
            LicenseKind::Unlicense => r#"
The Unlicense

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
            "#
            .to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[clap(rename_all = "lower")]
enum LicensePermission {
    Permissive,
    Copyleft,
    #[clap(name = "public-domain")]
    PublicDomain,
}

impl LicensePermission {
    fn as_str(&self) -> &'static str {
        match self {
            LicensePermission::Permissive => "Permissive",
            LicensePermission::Copyleft => "Copyleft",
            LicensePermission::PublicDomain => "Public Domain",
        }
    }
}
