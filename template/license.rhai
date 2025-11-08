
switch variable::get("license") {
    "AGPL" => {
        file::rename("LICENSE-AGPL", "LICENSE");
        file::delete("LICENSE-APACHE");
        file::delete("LICENSE-GPL");
        file::delete("LICENSE-LGPL");
        file::delete("LICENSE-MIT");
        variable::set("license", "AGPL-3.0-or-later");
        variable::set("prefix", " Affero ");
    }
    "GPL" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-APACHE");
        file::rename("LICENSE-GPL", "LICENSE");
        file::delete("LICENSE-LGPL");
        file::delete("LICENSE-MIT");
        variable::set("license", "GPL-3.0-or-later");
        variable::set("prefix", " ");
    }
    "LGPL" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-APACHE");
        file::delete("LICENSE-GPL");
        file::rename("LICENSE-LGPL", "LICENSE");
        file::delete("LICENSE-MIT");
        variable::set("license", "LGPL-3.0-or-later");
        variable::set("prefix", " Lesser ");
    }
    "MIT OR Apache" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-GPL");
        file::delete("LICENSE-LGPL");
        variable::set("license", "MIT OR Apache-2.0");
    }
    "MIT" => {
        file::delete("LICENSE-AGPL");
        file::delete("LICENSE-APACHE");
        file::delete("LICENSE-GPL");
        file::delete("LICENSE-LGPL");
        file::rename("LICENSE-MIT", "LICENSE");
        variable::set("license", "MIT");
    }
}
