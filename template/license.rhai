
switch variable::get("license") {
    "EUPL" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-APACHE");
        file::rename("LICENSE-EUPL", "LICENSE");
        file::delete("LICENSE-GPL");
        file::delete("LICENSE-LGPL");
        file::delete("LICENSE-MIT");
        variable::set("license", "EUPL-1.2");
        file::rename("README-EUPL.md", "README.md");
        file::delete("README-GPL.md");
        file::delete("README-MIT-OR-APACHE.md");
    }
    "AGPL" => {
        file::rename("LICENSE-AGPL", "LICENSE");
        file::delete("LICENSE-APACHE");
        file::delete("LICENSE-EUPL");
        file::delete("LICENSE-GPL");
        file::delete("LICENSE-LGPL");
        file::delete("LICENSE-MIT");
        variable::set("license", "AGPL-3.0-or-later");
        variable::set("prefix", " Affero ");
        file::delete("README-EUPL.md");
        file::rename("README-GPL.md", "README.md");
        file::delete("README-MIT-OR-APACHE.md");
    }
    "GPL" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-APACHE");
        file::delete("LICENSE-EUPL");
        file::rename("LICENSE-GPL", "LICENSE");
        file::delete("LICENSE-LGPL");
        file::delete("LICENSE-MIT");
        variable::set("license", "GPL-3.0-or-later");
        variable::set("prefix", " ");
        file::delete("README-EUPL.md");
        file::rename("README-GPL.md", "README.md");
        file::delete("README-MIT-OR-APACHE.md");
    }
    "LGPL" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-APACHE");
        file::delete("LICENSE-EUPL");
        file::delete("LICENSE-GPL");
        file::rename("LICENSE-LGPL", "LICENSE");
        file::delete("LICENSE-MIT");
        variable::set("license", "LGPL-3.0-or-later");
        variable::set("prefix", " Lesser ");
        file::delete("README-EUPL.md");
        file::rename("README-GPL.md", "README.md");
        file::delete("README-MIT-OR-APACHE.md");
    }
    "MIT OR Apache" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-EUPL");
        file::delete("LICENSE-GPL");
        file::delete("LICENSE-LGPL");
        variable::set("license", "MIT OR Apache-2.0");
        file::delete("README-EUPL.md");
        file::delete("README-GPL.md");
        file::rename("README-MIT-OR-APACHE.md", "README.md");
    }
    "MIT" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-APACHE");
        file::delete("LICENSE-EUPL");
        file::delete("LICENSE-GPL");
        file::delete("LICENSE-LGPL");
        file::rename("LICENSE-MIT", "LICENSE");
        variable::set("license", "MIT");
        file::delete("README-EUPL.md");
        file::delete("README-GPL.md");
        file::delete("README-MIT-OR-APACHE.md");
    }
}
