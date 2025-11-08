## License
{% case license %}
  {% when "MIT OR Apache-2.0" %}
Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
  {% else %}
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU{{ prefix }}General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU{{ prefix }}General Public License for more details.

You should have received a copy of the GNU{{ prefix }}General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.{% endcase %}{% if license == "MIT OR Apache-2.0" %}
## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.{% endif %}
