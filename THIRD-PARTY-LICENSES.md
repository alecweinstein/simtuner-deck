# Third-party licenses

Sim Tuner Deck is GPL-3.0-or-later. See LICENSE.md and NOTICE.

The host binary links these notable components:

| Component | License | Source |
| --- | --- | --- |
| OpenDeck | GPL-3.0-or-later | https://github.com/nekename/OpenDeck |
| elgato-streamdeck 0.14 | MPL-2.0 | https://github.com/OpenActionAPI/rust-elgato-streamdeck |
| Tauri, serde, tokio, image, and the other crates in src-tauri/Cargo.lock | MIT, Apache-2.0, MIT OR Apache-2.0, or the SPDX expression recorded on that crate | crates.io (repository field of each crate) |

The bundled Ajazz AKP153 plugin (GPL-3.0-or-later, source in NOTICE) links **mirajazz**, which is MPL-2.0:
https://github.com/4ndv/mirajazz

MPL-2.0 source for unmodified library dependencies is the tagged upstream revision those crates pin. This install does not vendor every crate's full source tree; the corresponding source is the public repository and version named here and in Cargo.lock.

The MIT, Apache-2.0, and MPL-2.0 license texts follow. Per-crate copyright notices are in each crate's published files at the version in Cargo.lock. A machine-readable list of direct and transitive Rust dependencies is appended when generated from `cargo metadata`.


## MIT

MIT License

Copyright (c) <year> <copyright holders>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the
following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT
LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO
EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
USE OR OTHER DEALINGS IN THE SOFTWARE.

## Apache-2.0


                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright [yyyy] [name of copyright owner]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

## MPL-2.0

Mozilla Public License Version 2.0
==================================

1. Definitions
--------------

1.1. "Contributor"
    means each individual or legal entity that creates, contributes to
    the creation of, or owns Covered Software.

1.2. "Contributor Version"
    means the combination of the Contributions of others (if any) used
    by a Contributor and that particular Contributor's Contribution.

1.3. "Contribution"
    means Covered Software of a particular Contributor.

1.4. "Covered Software"
    means Source Code Form to which the initial Contributor has attached
    the notice in Exhibit A, the Executable Form of such Source Code
    Form, and Modifications of such Source Code Form, in each case
    including portions thereof.

1.5. "Incompatible With Secondary Licenses"
    means

    (a) that the initial Contributor has attached the notice described
        in Exhibit B to the Covered Software; or

    (b) that the Covered Software was made available under the terms of
        version 1.1 or earlier of the License, but not also under the
        terms of a Secondary License.

1.6. "Executable Form"
    means any form of the work other than Source Code Form.

1.7. "Larger Work"
    means a work that combines Covered Software with other material, in
    a separate file or files, that is not Covered Software.

1.8. "License"
    means this document.

1.9. "Licensable"
    means having the right to grant, to the maximum extent possible,
    whether at the time of the initial grant or subsequently, any and
    all of the rights conveyed by this License.

1.10. "Modifications"
    means any of the following:

    (a) any file in Source Code Form that results from an addition to,
        deletion from, or modification of the contents of Covered
        Software; or

    (b) any new file in Source Code Form that contains any Covered
        Software.

1.11. "Patent Claims" of a Contributor
    means any patent claim(s), including without limitation, method,
    process, and apparatus claims, in any patent Licensable by such
    Contributor that would be infringed, but for the grant of the
    License, by the making, using, selling, offering for sale, having
    made, import, or transfer of either its Contributions or its
    Contributor Version.

1.12. "Secondary License"
    means either the GNU General Public License, Version 2.0, the GNU
    Lesser General Public License, Version 2.1, the GNU Affero General
    Public License, Version 3.0, or any later versions of those
    licenses.

1.13. "Source Code Form"
    means the form of the work preferred for making modifications.

1.14. "You" (or "Your")
    means an individual or a legal entity exercising rights under this
    License. For legal entities, "You" includes any entity that
    controls, is controlled by, or is under common control with You. For
    purposes of this definition, "control" means (a) the power, direct
    or indirect, to cause the direction or management of such entity,
    whether by contract or otherwise, or (b) ownership of more than
    fifty percent (50%) of the outstanding shares or beneficial
    ownership of such entity.

2. License Grants and Conditions
--------------------------------

2.1. Grants

Each Contributor hereby grants You a world-wide, royalty-free,
non-exclusive license:

(a) under intellectual property rights (other than patent or trademark)
    Licensable by such Contributor to use, reproduce, make available,
    modify, display, perform, distribute, and otherwise exploit its
    Contributions, either on an unmodified basis, with Modifications, or
    as part of a Larger Work; and

(b) under Patent Claims of such Contributor to make, use, sell, offer
    for sale, have made, import, and otherwise transfer either its
    Contributions or its Contributor Version.

2.2. Effective Date

The licenses granted in Section 2.1 with respect to any Contribution
become effective for each Contribution on the date the Contributor first
distributes such Contribution.

2.3. Limitations on Grant Scope

The licenses granted in this Section 2 are the only rights granted under
this License. No additional rights or licenses will be implied from the
distribution or licensing of Covered Software under this License.
Notwithstanding Section 2.1(b) above, no patent license is granted by a
Contributor:

(a) for any code that a Contributor has removed from Covered Software;
    or

(b) for infringements caused by: (i) Your and any other third party's
    modifications of Covered Software, or (ii) the combination of its
    Contributions with other software (except as part of its Contributor
    Version); or

(c) under Patent Claims infringed by Covered Software in the absence of
    its Contributions.

This License does not grant any rights in the trademarks, service marks,
or logos of any Contributor (except as may be necessary to comply with
the notice requirements in Section 3.4).

2.4. Subsequent Licenses

No Contributor makes additional grants as a result of Your choice to
distribute the Covered Software under a subsequent version of this
License (see Section 10.2) or under the terms of a Secondary License (if
permitted under the terms of Section 3.3).

2.5. Representation

Each Contributor represents that the Contributor believes its
Contributions are its original creation(s) or it has sufficient rights
to grant the rights to its Contributions conveyed by this License.

2.6. Fair Use

This License is not intended to limit any rights You have under
applicable copyright doctrines of fair use, fair dealing, or other
equivalents.

2.7. Conditions

Sections 3.1, 3.2, 3.3, and 3.4 are conditions of the licenses granted
in Section 2.1.

3. Responsibilities
-------------------

3.1. Distribution of Source Form

All distribution of Covered Software in Source Code Form, including any
Modifications that You create or to which You contribute, must be under
the terms of this License. You must inform recipients that the Source
Code Form of the Covered Software is governed by the terms of this
License, and how they can obtain a copy of this License. You may not
attempt to alter or restrict the recipients' rights in the Source Code
Form.

3.2. Distribution of Executable Form

If You distribute Covered Software in Executable Form then:

(a) such Covered Software must also be made available in Source Code
    Form, as described in Section 3.1, and You must inform recipients of
    the Executable Form how they can obtain a copy of such Source Code
    Form by reasonable means in a timely manner, at a charge no more
    than the cost of distribution to the recipient; and

(b) You may distribute such Executable Form under the terms of this
    License, or sublicense it under different terms, provided that the
    license for the Executable Form does not attempt to limit or alter
    the recipients' rights in the Source Code Form under this License.

3.3. Distribution of a Larger Work

You may create and distribute a Larger Work under terms of Your choice,
provided that You also comply with the requirements of this License for
the Covered Software. If the Larger Work is a combination of Covered
Software with a work governed by one or more Secondary Licenses, and the
Covered Software is not Incompatible With Secondary Licenses, this
License permits You to additionally distribute such Covered Software
under the terms of such Secondary License(s), so that the recipient of
the Larger Work may, at their option, further distribute the Covered
Software under the terms of either this License or such Secondary
License(s).

3.4. Notices

You may not remove or alter the substance of any license notices
(including copyright notices, patent notices, disclaimers of warranty,
or limitations of liability) contained within the Source Code Form of
the Covered Software, except that You may alter any license notices to
the extent required to remedy known factual inaccuracies.

3.5. Application of Additional Terms

You may choose to offer, and to charge a fee for, warranty, support,
indemnity or liability obligations to one or more recipients of Covered
Software. However, You may do so only on Your own behalf, and not on
behalf of any Contributor. You must make it absolutely clear that any
such warranty, support, indemnity, or liability obligation is offered by
You alone, and You hereby agree to indemnify every Contributor for any
liability incurred by such Contributor as a result of warranty, support,
indemnity or liability terms You offer. You may include additional
disclaimers of warranty and limitations of liability specific to any
jurisdiction.

4. Inability to Comply Due to Statute or Regulation
---------------------------------------------------

If it is impossible for You to comply with any of the terms of this
License with respect to some or all of the Covered Software due to
statute, judicial order, or regulation then You must: (a) comply with
the terms of this License to the maximum extent possible; and (b)
describe the limitations and the code they affect. Such description must
be placed in a text file included with all distributions of the Covered
Software under this License. Except to the extent prohibited by statute
or regulation, such description must be sufficiently detailed for a
recipient of ordinary skill to be able to understand it.

5. Termination
--------------

5.1. The rights granted under this License will terminate automatically
if You fail to comply with any of its terms. However, if You become
compliant, then the rights granted under this License from a particular
Contributor are reinstated (a) provisionally, unless and until such
Contributor explicitly and finally terminates Your grants, and (b) on an
ongoing basis, if such Contributor fails to notify You of the
non-compliance by some reasonable means prior to 60 days after You have
come back into compliance. Moreover, Your grants from a particular
Contributor are reinstated on an ongoing basis if such Contributor
notifies You of the non-compliance by some reasonable means, this is the
first time You have received notice of non-compliance with this License
from such Contributor, and You become compliant prior to 30 days after
Your receipt of the notice.

5.2. If You initiate litigation against any entity by asserting a patent
infringement claim (excluding declaratory judgment actions,
counter-claims, and cross-claims) alleging that a Contributor Version
directly or indirectly infringes any patent, then the rights granted to
You by any and all Contributors for the Covered Software under Section
2.1 of this License shall terminate.

5.3. In the event of termination under Sections 5.1 or 5.2 above, all
end user license agreements (excluding distributors and resellers) which
have been validly granted by You or Your distributors under this License
prior to termination shall survive termination.

************************************************************************
*                                                                      *
*  6. Disclaimer of Warranty                                           *
*  -------------------------                                           *
*                                                                      *
*  Covered Software is provided under this License on an "as is"       *
*  basis, without warranty of any kind, either expressed, implied, or  *
*  statutory, including, without limitation, warranties that the       *
*  Covered Software is free of defects, merchantable, fit for a        *
*  particular purpose or non-infringing. The entire risk as to the     *
*  quality and performance of the Covered Software is with You.        *
*  Should any Covered Software prove defective in any respect, You     *
*  (not any Contributor) assume the cost of any necessary servicing,   *
*  repair, or correction. This disclaimer of warranty constitutes an   *
*  essential part of this License. No use of any Covered Software is   *
*  authorized under this License except under this disclaimer.         *
*                                                                      *
************************************************************************

************************************************************************
*                                                                      *
*  7. Limitation of Liability                                          *
*  --------------------------                                          *
*                                                                      *
*  Under no circumstances and under no legal theory, whether tort      *
*  (including negligence), contract, or otherwise, shall any           *
*  Contributor, or anyone who distributes Covered Software as          *
*  permitted above, be liable to You for any direct, indirect,         *
*  special, incidental, or consequential damages of any character      *
*  including, without limitation, damages for lost profits, loss of    *
*  goodwill, work stoppage, computer failure or malfunction, or any    *
*  and all other commercial damages or losses, even if such party      *
*  shall have been informed of the possibility of such damages. This   *
*  limitation of liability shall not apply to liability for death or   *
*  personal injury resulting from such party's negligence to the       *
*  extent applicable law prohibits such limitation. Some               *
*  jurisdictions do not allow the exclusion or limitation of           *
*  incidental or consequential damages, so this exclusion and          *
*  limitation may not apply to You.                                    *
*                                                                      *
************************************************************************

8. Litigation
-------------

Any litigation relating to this License may be brought only in the
courts of a jurisdiction where the defendant maintains its principal
place of business and such litigation shall be governed by laws of that
jurisdiction, without reference to its conflict-of-law provisions.
Nothing in this Section shall prevent a party's ability to bring
cross-claims or counter-claims.

9. Miscellaneous
----------------

This License represents the complete agreement concerning the subject
matter hereof. If any provision of this License is held to be
unenforceable, such provision shall be reformed only to the extent
necessary to make it enforceable. Any law or regulation which provides
that the language of a contract shall be construed against the drafter
shall not be used to construe this License against a Contributor.

10. Versions of the License
---------------------------

10.1. New Versions

Mozilla Foundation is the license steward. Except as provided in Section
10.3, no one other than the license steward has the right to modify or
publish new versions of this License. Each version will be given a
distinguishing version number.

10.2. Effect of New Versions

You may distribute the Covered Software under the terms of the version
of the License under which You originally received the Covered Software,
or under the terms of any subsequent version published by the license
steward.

10.3. Modified Versions

If you create software not governed by this License, and you want to
create a new license for such software, you may create and use a
modified version of this License if you rename the license and remove
any references to the name of the license steward (except to note that
such modified license differs from this License).

10.4. Distributing Source Code Form that is Incompatible With Secondary
Licenses

If You choose to distribute Source Code Form that is Incompatible With
Secondary Licenses under the terms of this version of the License, the
notice described in Exhibit B of this License must be attached.

Exhibit A - Source Code Form License Notice
-------------------------------------------

  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at https://mozilla.org/MPL/2.0/.

If it is not possible or desirable to put the notice in a particular
file, then You may include the notice in a location (such as a LICENSE
file in a relevant directory) where a recipient would be likely to look
for such a notice.

You may add additional accurate notices of copyright ownership.

Exhibit B - "Incompatible With Secondary Licenses" Notice
---------------------------------------------------------

  This Source Code Form is "Incompatible With Secondary Licenses", as
  defined by the Mozilla Public License, v. 2.0.

## Rust dependency licenses

Generated from `cargo metadata` for this revision. The list is every crate in Cargo.lock, including crates only built for other platforms, not only the ones linked into the Windows setup.

- ab_glyph 0.2.32 — Apache-2.0 — https://github.com/alexheretic/ab-glyph
- ab_glyph_rasterizer 0.1.10 — Apache-2.0 — https://github.com/alexheretic/ab-glyph
- active-win-pos-rs 0.10.1 — MIT OR Apache-2.0 — https://github.com/dimusic/active-win-pos-rs
- adler2 2.0.1 — 0BSD OR MIT OR Apache-2.0 — https://github.com/oyvindln/adler2
- ahash 0.7.8 — MIT OR Apache-2.0 — https://github.com/tkaitchuck/ahash
- aho-corasick 1.1.4 — Unlicense OR MIT — https://github.com/BurntSushi/aho-corasick
- alloc-no-stdlib 3.0.0 — BSD-3-Clause — https://github.com/dropbox/rust-alloc-no-stdlib
- alloc-stdlib 0.3.0 — BSD-3-Clause — https://github.com/dropbox/rust-alloc-no-stdlib
- android_log-sys 0.3.2 — MIT OR Apache-2.0 — https://github.com/rust-mobile/android_log-sys-rs
- android_logger 0.15.1 — MIT OR Apache-2.0 — https://github.com/rust-mobile/android_logger-rs
- android_system_properties 0.1.5 — MIT/Apache-2.0 — https://github.com/nical/android_system_properties
- anyhow 1.0.101 — MIT OR Apache-2.0 — https://github.com/dtolnay/anyhow
- appkit-nsworkspace-bindings 0.1.2 — MIT OR Apache-2.0 — https://github.com/dimusic/active-win-pos-rs
- arbitrary 1.4.2 — MIT OR Apache-2.0 — https://github.com/rust-fuzz/arbitrary/
- arrayref 0.3.9 — BSD-2-Clause — https://github.com/droundy/arrayref
- arrayvec 0.7.6 — MIT OR Apache-2.0 — https://github.com/bluss/arrayvec
- ascii 1.1.0 — Apache-2.0 OR MIT — https://github.com/tomprogrammer/rust-ascii
- async-broadcast 0.7.2 — MIT OR Apache-2.0 — https://github.com/smol-rs/async-broadcast
- async-channel 2.5.0 — Apache-2.0 OR MIT — https://github.com/smol-rs/async-channel
- async-executor 1.13.3 — Apache-2.0 OR MIT — https://github.com/smol-rs/async-executor
- async-io 2.6.0 — Apache-2.0 OR MIT — https://github.com/smol-rs/async-io
- async-lock 3.4.2 — Apache-2.0 OR MIT — https://github.com/smol-rs/async-lock
- async-process 2.5.0 — Apache-2.0 OR MIT — https://github.com/smol-rs/async-process
- async-recursion 1.1.1 — MIT OR Apache-2.0 — https://github.com/dcchut/async-recursion
- async-signal 0.2.13 — Apache-2.0 OR MIT — https://github.com/smol-rs/async-signal
- async-task 4.7.1 — Apache-2.0 OR MIT — https://github.com/smol-rs/async-task
- async-trait 0.1.89 — MIT OR Apache-2.0 — https://github.com/dtolnay/async-trait
- atk 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- atk-sys 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- atomic-waker 1.1.2 — Apache-2.0 OR MIT — https://github.com/smol-rs/atomic-waker
- auto-launch 0.5.0 — MIT — https://github.com/zzzgydi/auto-launch.git
- autocfg 1.5.0 — Apache-2.0 OR MIT — https://github.com/cuviper/autocfg
- aws-lc-rs 1.15.4 — ISC AND (Apache-2.0 OR ISC) — https://github.com/aws/aws-lc-rs
- aws-lc-sys 0.37.0 — ISC AND (Apache-2.0 OR ISC) AND OpenSSL — https://github.com/aws/aws-lc-rs
- base64 0.21.7 — MIT OR Apache-2.0 — https://github.com/marshallpierce/rust-base64
- base64 0.22.1 — MIT OR Apache-2.0 — https://github.com/marshallpierce/rust-base64
- bindgen 0.68.1 — BSD-3-Clause — https://github.com/rust-lang/rust-bindgen
- bit-set 0.8.0 — Apache-2.0 OR MIT — https://github.com/contain-rs/bit-set
- bit-vec 0.8.0 — Apache-2.0 OR MIT — https://github.com/contain-rs/bit-vec
- bitflags 1.3.2 — MIT/Apache-2.0 — https://github.com/bitflags/bitflags
- bitflags 2.10.0 — MIT OR Apache-2.0 — https://github.com/bitflags/bitflags
- bitvec 1.0.1 — MIT — https://github.com/bitvecto-rs/bitvec
- block 0.1.6 — MIT — http://github.com/SSheldon/rust-block
- block-buffer 0.10.4 — MIT OR Apache-2.0 — https://github.com/RustCrypto/utils
- block2 0.6.2 — MIT — https://github.com/madsmtm/objc2
- blocking 1.6.2 — Apache-2.0 OR MIT — https://github.com/smol-rs/blocking
- borsh 1.6.0 — MIT OR Apache-2.0 — https://github.com/near/borsh-rs
- borsh-derive 1.6.0 — Apache-2.0 — https://github.com/near/borsh-rs
- brotli 9.0.0 — BSD-3-Clause AND MIT — https://github.com/dropbox/rust-brotli
- brotli-decompressor 6.0.1 — BSD-3-Clause/MIT — https://github.com/dropbox/rust-brotli-decompressor
- built 0.8.0 — MIT — https://github.com/lukaslueg/built
- bumpalo 3.19.1 — MIT OR Apache-2.0 — https://github.com/fitzgen/bumpalo
- byte-unit 5.2.0 — MIT — https://github.com/magiclen/byte-unit
- bytecheck 0.6.12 — MIT — https://github.com/djkoloski/bytecheck
- bytecheck_derive 0.6.12 — MIT — https://github.com/djkoloski/bytecheck
- bytemuck 1.25.0 — Zlib OR Apache-2.0 OR MIT — https://github.com/Lokathor/bytemuck
- byteorder 1.5.0 — Unlicense OR MIT — https://github.com/BurntSushi/byteorder
- byteorder-lite 0.1.0 — Unlicense OR MIT — https://github.com/image-rs/byteorder-lite
- bytes 1.11.1 — MIT — https://github.com/tokio-rs/bytes
- cairo-rs 0.18.5 — MIT — https://github.com/gtk-rs/gtk-rs-core
- cairo-sys-rs 0.18.2 — MIT — https://github.com/gtk-rs/gtk-rs-core
- camino 1.2.2 — MIT OR Apache-2.0 — https://github.com/camino-rs/camino
- cargo-lock 10.1.0 — Apache-2.0 OR MIT — https://github.com/rustsec/rustsec
- cargo-platform 0.1.9 — MIT OR Apache-2.0 — https://github.com/rust-lang/cargo
- cargo_metadata 0.19.2 — MIT — https://github.com/oli-obk/cargo_metadata
- cargo_toml 1.0.1 — Apache-2.0 OR MIT — https://gitlab.com/lib.rs/cargo_toml
- cc 1.2.55 — MIT OR Apache-2.0 — https://github.com/rust-lang/cc-rs
- cesu8 1.1.0 — Apache-2.0/MIT — https://github.com/emk/cesu8-rs
- cexpr 0.6.0 — Apache-2.0/MIT — https://github.com/jethrogb/rust-cexpr
- cfb 0.14.0 — MIT — https://github.com/mdsteele/rust-cfb
- cfg-expr 0.15.8 — MIT OR Apache-2.0 — https://github.com/EmbarkStudios/cfg-expr
- cfg-if 1.0.4 — MIT OR Apache-2.0 — https://github.com/rust-lang/cfg-if
- cfg_aliases 0.2.1 — MIT — https://github.com/katharostech/cfg_aliases
- chrono 0.4.43 — MIT OR Apache-2.0 — https://github.com/chronotope/chrono
- chunked_transfer 1.5.0 — MIT OR Apache-2.0 — https://github.com/frewsxcv/rust-chunked-transfer
- clang-sys 1.8.1 — Apache-2.0 — https://github.com/KyleMayes/clang-sys
- cmake 0.1.57 — MIT OR Apache-2.0 — https://github.com/rust-lang/cmake-rs
- cocoa 0.26.1 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- cocoa-foundation 0.2.1 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- combine 4.6.7 — MIT — https://github.com/Marwes/combine
- concurrent-queue 2.5.0 — Apache-2.0 OR MIT — https://github.com/smol-rs/concurrent-queue
- const-random 0.1.18 — MIT OR Apache-2.0 — https://github.com/tkaitchuck/constrandom
- const-random-macro 0.1.16 — MIT OR Apache-2.0 — https://github.com/tkaitchuck/constrandom
- convert_case 0.4.0 — MIT — https://github.com/rutrum/convert-case
- cookie 0.18.1 — MIT OR Apache-2.0 — https://github.com/SergioBenitez/cookie-rs
- cookie_store 0.22.0 — MIT OR Apache-2.0 — https://github.com/pfernie/cookie_store
- core-foundation 0.10.1 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-foundation 0.9.4 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-foundation-sys 0.8.7 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-graphics 0.22.3 — MIT / Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-graphics 0.23.2 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-graphics 0.24.0 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-graphics 0.25.0 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-graphics-types 0.1.3 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-graphics-types 0.2.0 — MIT OR Apache-2.0 — https://github.com/servo/core-foundation-rs
- core-text 19.2.0 — MIT/Apache-2.0 — https://github.com/servo/core-foundation-rs
- core_maths 0.1.1 — MIT — https://github.com/robertbastian/core_maths
- cpufeatures 0.2.17 — MIT OR Apache-2.0 — https://github.com/RustCrypto/utils
- crc32fast 1.5.0 — MIT OR Apache-2.0 — https://github.com/srijs/rust-crc32fast
- crossbeam-channel 0.5.15 — MIT OR Apache-2.0 — https://github.com/crossbeam-rs/crossbeam
- crossbeam-utils 0.8.21 — MIT OR Apache-2.0 — https://github.com/crossbeam-rs/crossbeam
- crunchy 0.2.4 — MIT — https://github.com/eira-fransham/crunchy
- crypto-common 0.1.7 — MIT OR Apache-2.0 — https://github.com/RustCrypto/traits
- cssparser 0.29.6 — MPL-2.0 — https://github.com/servo/rust-cssparser
- cssparser 0.37.0 — MPL-2.0 — https://github.com/servo/rust-cssparser
- cssparser-macros 0.6.1 — MPL-2.0 — https://github.com/servo/rust-cssparser
- cssparser-macros 0.7.1 — MPL-2.0 — https://github.com/servo/rust-cssparser
- ctor 1.0.13 — Apache-2.0 OR MIT — https://github.com/mmastrac/linktime
- darling 0.20.11 — MIT — https://github.com/TedDriggs/darling
- darling 0.21.3 — MIT — https://github.com/TedDriggs/darling
- darling_core 0.20.11 — MIT — https://github.com/TedDriggs/darling
- darling_core 0.21.3 — MIT — https://github.com/TedDriggs/darling
- darling_macro 0.20.11 — MIT — https://github.com/TedDriggs/darling
- darling_macro 0.21.3 — MIT — https://github.com/TedDriggs/darling
- dashmap 6.1.0 — MIT — https://github.com/xacrimon/dashmap
- data-encoding 2.10.0 — MIT — https://github.com/ia0/data-encoding
- data-url 0.3.2 — MIT OR Apache-2.0 — https://github.com/servo/rust-url
- dbus 0.9.10 — Apache-2.0/MIT — https://github.com/diwic/dbus-rs
- deranged 0.5.5 — MIT OR Apache-2.0 — https://github.com/jhpratt/deranged
- derive_arbitrary 1.4.2 — MIT OR Apache-2.0 — https://github.com/rust-fuzz/arbitrary
- derive_builder 0.20.2 — MIT OR Apache-2.0 — https://github.com/colin-kiegel/rust-derive-builder
- derive_builder_core 0.20.2 — MIT OR Apache-2.0 — https://github.com/colin-kiegel/rust-derive-builder
- derive_builder_macro 0.20.2 — MIT OR Apache-2.0 — https://github.com/colin-kiegel/rust-derive-builder
- derive_more 0.99.20 — MIT — https://github.com/JelteF/derive_more
- derive_more 2.1.1 — MIT — https://github.com/JelteF/derive_more
- derive_more-impl 2.1.1 — MIT — https://github.com/JelteF/derive_more
- digest 0.10.7 — MIT OR Apache-2.0 — https://github.com/RustCrypto/traits
- dirs 4.0.0 — MIT OR Apache-2.0 — https://github.com/soc/dirs-rs
- dirs 7.0.0 — MIT OR Apache-2.0 — https://codeberg.org/dirs/dirs-rs
- dirs-sys 0.3.7 — MIT OR Apache-2.0 — https://github.com/dirs-dev/dirs-sys-rs
- dirs-sys 0.5.0 — MIT OR Apache-2.0 — https://github.com/dirs-dev/dirs-sys-rs
- dispatch2 0.3.0 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- displaydoc 0.2.5 — MIT OR Apache-2.0 — https://github.com/yaahc/displaydoc
- dlopen2 0.8.2 — MIT — https://github.com/OpenByteDev/dlopen2
- dlopen2_derive 0.4.3 — MIT — https://github.com/OpenByteDev/dlopen2
- dlv-list 0.5.2 — MIT OR Apache-2.0 — https://github.com/sgodwincs/dlv-list-rs
- document-features 0.2.12 — MIT OR Apache-2.0 — https://github.com/slint-ui/document-features
- dom_query 0.28.0 — MIT — https://github.com/niklak/dom_query
- dpi 0.1.2 — Apache-2.0 AND MIT — https://github.com/rust-windowing/winit
- dtoa 1.0.11 — MIT OR Apache-2.0 — https://github.com/dtolnay/dtoa
- dtoa-short 0.3.5 — MPL-2.0 — https://github.com/upsuper/dtoa-short
- dunce 1.0.5 — CC0-1.0 OR MIT-0 OR Apache-2.0 — https://gitlab.com/kornelski/dunce
- dyn-clone 1.0.20 — MIT OR Apache-2.0 — https://github.com/dtolnay/dyn-clone
- either 1.15.0 — MIT OR Apache-2.0 — https://github.com/rayon-rs/either
- elgato-streamdeck 0.14.0 — MPL-2.0 — https://github.com/OpenActionAPI/rust-elgato-streamdeck
- embed-resource 3.0.6 — MIT — https://github.com/nabijaczleweli/rust-embed-resource
- embed_plist 1.2.2 — MIT OR Apache-2.0 — https://github.com/nvzqz/embed-plist-rs
- encoding_rs 0.8.35 — (Apache-2.0 OR MIT) AND BSD-3-Clause — https://github.com/hsivonen/encoding_rs
- endi 1.1.1 — MIT — https://github.com/zeenix/endi
- enumflags2 0.7.12 — MIT OR Apache-2.0 — https://github.com/meithecatte/enumflags2
- enumflags2_derive 0.7.12 — MIT OR Apache-2.0 — https://github.com/meithecatte/enumflags2
- env_filter 0.1.4 — MIT OR Apache-2.0 — https://github.com/rust-cli/env_logger
- equivalent 1.0.2 — Apache-2.0 OR MIT — https://github.com/indexmap-rs/equivalent
- erased-serde 0.4.9 — MIT OR Apache-2.0 — https://github.com/dtolnay/erased-serde
- errno 0.3.14 — MIT OR Apache-2.0 — https://github.com/lambda-fairy/rust-errno
- euclid 0.22.14 — MIT OR Apache-2.0 — https://github.com/servo/euclid
- event-listener 5.4.1 — Apache-2.0 OR MIT — https://github.com/smol-rs/event-listener
- event-listener-strategy 0.5.4 — Apache-2.0 OR MIT — https://github.com/smol-rs/event-listener-strategy
- expat-sys 2.1.6 — MIT — https://github.com/servo/libexpat/
- fastrand 2.3.0 — Apache-2.0 OR MIT — https://github.com/smol-rs/fastrand
- fdeflate 0.3.7 — MIT OR Apache-2.0 — https://github.com/image-rs/fdeflate
- fern 0.7.1 — MIT — https://github.com/daboross/fern
- field-offset 0.3.6 — MIT OR Apache-2.0 — https://github.com/Diggsey/rust-field-offset
- find-msvc-tools 0.1.9 — MIT OR Apache-2.0 — https://github.com/rust-lang/cc-rs
- fix-path-env 0.0.0 — Apache-2.0 OR MIT — 
- fixedbitset 0.4.2 — MIT/Apache-2.0 — https://github.com/petgraph/fixedbitset
- flate2 1.1.9 — MIT OR Apache-2.0 — https://github.com/rust-lang/flate2-rs
- float-cmp 0.9.0 — MIT — https://github.com/mikedilger/float-cmp
- fnv 1.0.7 — Apache-2.0 / MIT — https://github.com/servo/rust-fnv
- foldhash 0.2.0 — Zlib — https://github.com/orlp/foldhash
- font-loader 0.11.0 — MIT — https://github.com/matprec/rust-font-loader
- fontconfig-parser 0.5.8 — MIT — https://github.com/Riey/fontconfig-parser
- fontdb 0.23.0 — MIT — https://github.com/RazrFalcon/fontdb
- foreign-types 0.3.2 — MIT/Apache-2.0 — https://github.com/sfackler/foreign-types
- foreign-types 0.5.0 — MIT/Apache-2.0 — https://github.com/sfackler/foreign-types
- foreign-types-macros 0.2.3 — MIT/Apache-2.0 — https://github.com/sfackler/foreign-types
- foreign-types-shared 0.1.1 — MIT/Apache-2.0 — https://github.com/sfackler/foreign-types
- foreign-types-shared 0.3.1 — MIT/Apache-2.0 — https://github.com/sfackler/foreign-types
- form_urlencoded 1.2.2 — MIT OR Apache-2.0 — https://github.com/servo/rust-url
- freetype-sys 0.13.1 — MIT — https://github.com/PistonDevelopers/freetype-sys.git
- fs_extra 1.3.0 — MIT — https://github.com/webdesus/fs_extra
- funty 2.0.0 — MIT — https://github.com/myrrlyn/funty
- futf 0.1.5 — MIT / Apache-2.0 — https://github.com/servo/futf
- futures 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-channel 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-core 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-executor 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-io 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-lite 2.6.1 — Apache-2.0 OR MIT — https://github.com/smol-rs/futures-lite
- futures-macro 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-sink 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-task 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- futures-util 0.3.31 — MIT OR Apache-2.0 — https://github.com/rust-lang/futures-rs
- fxhash 0.2.1 — Apache-2.0/MIT — https://github.com/cbreeden/fxhash
- gdk 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- gdk-pixbuf 0.18.5 — MIT — https://github.com/gtk-rs/gtk-rs-core
- gdk-pixbuf-sys 0.18.0 — MIT — https://github.com/gtk-rs/gtk-rs-core
- gdk-sys 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- gdkwayland-sys 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- gdkx11 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- gdkx11-sys 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- generic-array 0.14.7 — MIT — https://github.com/fizyk20/generic-array.git
- getrandom 0.1.16 — MIT OR Apache-2.0 — https://github.com/rust-random/getrandom
- getrandom 0.2.17 — MIT OR Apache-2.0 — https://github.com/rust-random/getrandom
- getrandom 0.3.4 — MIT OR Apache-2.0 — https://github.com/rust-random/getrandom
- gio 0.18.4 — MIT — https://github.com/gtk-rs/gtk-rs-core
- gio-sys 0.18.1 — MIT — https://github.com/gtk-rs/gtk-rs-core
- git2 0.20.4 — MIT OR Apache-2.0 — https://github.com/rust-lang/git2-rs
- glib 0.18.5 — MIT — https://github.com/gtk-rs/gtk-rs-core
- glib-macros 0.18.5 — MIT — https://github.com/gtk-rs/gtk-rs-core
- glib-sys 0.18.1 — MIT — https://github.com/gtk-rs/gtk-rs-core
- glob 0.3.3 — MIT OR Apache-2.0 — https://github.com/rust-lang/glob
- gobject-sys 0.18.0 — MIT — https://github.com/gtk-rs/gtk-rs-core
- gtk 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- gtk-sys 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- gtk3-macros 0.18.2 — MIT — https://github.com/gtk-rs/gtk3-rs
- h2 0.4.13 — MIT — https://github.com/hyperium/h2
- handlebars 6.4.0 — MIT — https://github.com/sunng87/handlebars-rust
- hashbrown 0.12.3 — MIT OR Apache-2.0 — https://github.com/rust-lang/hashbrown
- hashbrown 0.14.5 — MIT OR Apache-2.0 — https://github.com/rust-lang/hashbrown
- hashbrown 0.16.1 — MIT OR Apache-2.0 — https://github.com/rust-lang/hashbrown
- heck 0.4.1 — MIT OR Apache-2.0 — https://github.com/withoutboats/heck
- heck 0.5.0 — MIT OR Apache-2.0 — https://github.com/withoutboats/heck
- hermit-abi 0.5.2 — MIT OR Apache-2.0 — https://github.com/hermit-os/hermit-rs
- hex 0.4.3 — MIT OR Apache-2.0 — https://github.com/KokaKiwi/rust-hex
- hidapi 2.6.4 — MIT — https://github.com/ruabmbua/hidapi-rs
- home 0.5.12 — MIT OR Apache-2.0 — https://github.com/rust-lang/cargo
- html5ever 0.29.1 — MIT OR Apache-2.0 — https://github.com/servo/html5ever
- html5ever 0.39.0 — MIT OR Apache-2.0 — https://github.com/servo/html5ever
- http 1.4.0 — MIT OR Apache-2.0 — https://github.com/hyperium/http
- http-body 1.0.1 — MIT — https://github.com/hyperium/http-body
- http-body-util 0.1.3 — MIT — https://github.com/hyperium/http-body
- httparse 1.10.1 — MIT OR Apache-2.0 — https://github.com/seanmonstar/httparse
- httpdate 1.0.3 — MIT OR Apache-2.0 — https://github.com/pyfisch/httpdate
- hyper 1.8.1 — MIT — https://github.com/hyperium/hyper
- hyper-rustls 0.27.7 — Apache-2.0 OR ISC OR MIT — https://github.com/rustls/hyper-rustls
- hyper-util 0.1.20 — MIT — https://github.com/hyperium/hyper-util
- iana-time-zone 0.1.65 — MIT OR Apache-2.0 — https://github.com/strawlab/iana-time-zone
- iana-time-zone-haiku 0.1.2 — MIT OR Apache-2.0 — https://github.com/strawlab/iana-time-zone
- ico 0.5.0 — MIT — https://github.com/mdsteele/rust-ico
- icu_collections 2.1.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- icu_locale_core 2.1.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- icu_normalizer 2.1.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- icu_normalizer_data 2.1.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- icu_properties 2.1.2 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- icu_properties_data 2.1.2 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- icu_provider 2.1.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- ident_case 1.0.1 — MIT/Apache-2.0 — https://github.com/TedDriggs/ident_case
- idna 1.1.0 — MIT OR Apache-2.0 — https://github.com/servo/rust-url/
- idna_adapter 1.2.1 — Apache-2.0 OR MIT — https://github.com/hsivonen/idna_adapter
- image 0.25.9 — MIT OR Apache-2.0 — https://github.com/image-rs/image
- imagesize 0.14.0 — MIT — https://github.com/Roughsketch/imagesize
- indexmap 1.9.3 — Apache-2.0 OR MIT — https://github.com/bluss/indexmap
- indexmap 2.13.0 — Apache-2.0 OR MIT — https://github.com/indexmap-rs/indexmap
- infer 0.22.0 — MIT — https://github.com/bojand/infer
- ipnet 2.11.0 — MIT OR Apache-2.0 — https://github.com/krisprice/ipnet
- iri-string 0.7.10 — MIT OR Apache-2.0 — https://github.com/lo48576/iri-string
- is-docker 0.2.0 — MIT — https://github.com/TheLarkInn/is-docker
- is-wsl 0.4.0 — MIT — https://github.com/TheLarkInn/is-wsl
- itoa 1.0.17 — MIT OR Apache-2.0 — https://github.com/dtolnay/itoa
- javascriptcore-rs 1.1.2 — MIT — https://github.com/tauri-apps/javascriptcore-rs
- javascriptcore-rs-sys 1.1.1 — MIT — https://github.com/tauri-apps/javascriptcore-rs
- jni 0.21.1 — MIT/Apache-2.0 — https://github.com/jni-rs/jni-rs
- jni-sys 0.3.0 — MIT/Apache-2.0 — https://github.com/sfackler/rust-jni-sys
- jobserver 0.1.34 — MIT OR Apache-2.0 — https://github.com/rust-lang/jobserver-rs
- js-sys 0.3.85 — MIT OR Apache-2.0 — https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/js-sys
- json-patch 4.1.0 — MIT/Apache-2.0 — https://github.com/idubrov/json-patch
- jsonptr 0.7.1 — MIT OR Apache-2.0 — https://github.com/chanced/jsonptr
- kdotool 0.2.3 — Apache-2.0 — https://github.com/jinliu/kdotool
- keyboard-types 0.8.3 — MIT OR Apache-2.0 — https://github.com/rust-windowing/keyboard-types
- kuchikiki 0.8.8-speedreader — MIT — https://github.com/brave/kuchikiki
- kurbo 0.13.1 — Apache-2.0 OR MIT — https://github.com/linebender/kurbo
- lazy_static 1.5.0 — MIT OR Apache-2.0 — https://github.com/rust-lang-nursery/lazy-static.rs
- lazycell 1.3.0 — MIT/Apache-2.0 — https://github.com/indiv0/lazycell
- libappindicator 0.9.0 — Apache-2.0 OR MIT — 
- libappindicator-sys 0.9.0 — Apache-2.0 OR MIT — 
- libc 0.2.180 — MIT OR Apache-2.0 — https://github.com/rust-lang/libc
- libdbus-sys 0.2.7 — Apache-2.0/MIT — https://github.com/diwic/dbus-rs
- libgit2-sys 0.18.3+1.9.2 — MIT OR Apache-2.0 — https://github.com/rust-lang/git2-rs
- libloading 0.7.4 — ISC — https://github.com/nagisa/rust_libloading/
- libloading 0.8.9 — ISC — https://github.com/nagisa/rust_libloading/
- libm 0.2.16 — MIT — https://github.com/rust-lang/compiler-builtins
- libredox 0.1.12 — MIT — https://gitlab.redox-os.org/redox-os/libredox.git
- libz-sys 1.1.23 — MIT OR Apache-2.0 — https://github.com/rust-lang/libz-sys
- linux-raw-sys 0.11.0 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/sunfishcode/linux-raw-sys
- linux-raw-sys 0.4.15 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/sunfishcode/linux-raw-sys
- litemap 0.8.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- litrs 1.0.0 — MIT OR Apache-2.0 — https://github.com/LukasKalbertodt/litrs
- lock_api 0.4.14 — MIT OR Apache-2.0 — https://github.com/Amanieu/parking_lot
- log 0.4.29 — MIT OR Apache-2.0 — https://github.com/rust-lang/log
- log-panics 2.1.0 — MIT/Apache-2.0 — https://github.com/sfackler/rust-log-panics
- lru-slab 0.1.2 — MIT OR Apache-2.0 OR Zlib — https://github.com/Ralith/lru-slab
- mac 0.1.1 — MIT/Apache-2.0 — https://github.com/reem/rust-mac.git
- malloc_buf 0.0.6 — MIT — https://github.com/SSheldon/malloc_buf
- markup5ever 0.14.1 — MIT OR Apache-2.0 — https://github.com/servo/html5ever
- markup5ever 0.39.0 — MIT OR Apache-2.0 — https://github.com/servo/html5ever
- match_token 0.1.0 — MIT OR Apache-2.0 — https://github.com/servo/html5ever
- matches 0.1.10 — MIT — https://github.com/SimonSapin/rust-std-candidates
- memchr 2.7.6 — Unlicense OR MIT — https://github.com/BurntSushi/memchr
- memoffset 0.9.1 — MIT — https://github.com/Gilnaa/memoffset
- mime 0.3.17 — MIT OR Apache-2.0 — https://github.com/hyperium/mime
- minimal-lexical 0.2.1 — MIT/Apache-2.0 — https://github.com/Alexhuszagh/minimal-lexical
- miniz_oxide 0.8.9 — MIT OR Zlib OR Apache-2.0 — https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide
- mio 1.1.1 — MIT — https://github.com/tokio-rs/mio
- moxcms 0.7.11 — BSD-3-Clause OR Apache-2.0 — https://github.com/awxkee/moxcms.git
- muda 0.20.0 — Apache-2.0 OR MIT — https://github.com/tauri-apps/muda
- ndk 0.9.0 — MIT OR Apache-2.0 — https://github.com/rust-mobile/ndk
- ndk-context 0.1.1 — MIT OR Apache-2.0 — https://github.com/rust-windowing/android-ndk-rs
- ndk-sys 0.6.0+11769913 — MIT OR Apache-2.0 — https://github.com/rust-mobile/ndk
- new_debug_unreachable 1.0.6 — MIT — https://github.com/mbrubeck/rust-debug-unreachable
- nix 0.30.1 — MIT — https://github.com/nix-rust/nix
- nodrop 0.1.14 — MIT/Apache-2.0 — https://github.com/bluss/arrayvec
- nom 7.1.3 — MIT — https://github.com/Geal/nom
- ntapi 0.4.2 — Apache-2.0 OR MIT — https://github.com/MSxDOS/ntapi
- num-conv 0.2.0 — MIT OR Apache-2.0 — https://github.com/jhpratt/num-conv
- num-modular 0.6.1 — Apache-2.0 — https://github.com/cmpute/num-modular
- num-order 1.2.0 — Apache-2.0 — https://github.com/cmpute/num-order
- num-traits 0.2.19 — MIT OR Apache-2.0 — https://github.com/rust-num/num-traits
- num_enum 0.7.5 — BSD-3-Clause OR MIT OR Apache-2.0 — https://github.com/illicitonion/num_enum
- num_enum_derive 0.7.5 — BSD-3-Clause OR MIT OR Apache-2.0 — https://github.com/illicitonion/num_enum
- num_threads 0.1.7 — MIT OR Apache-2.0 — https://github.com/jhpratt/num_threads
- objc 0.2.7 — MIT — http://github.com/SSheldon/rust-objc
- objc2 0.6.4 — MIT — https://github.com/madsmtm/objc2
- objc2-app-kit 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-cloud-kit 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-core-data 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-core-foundation 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-core-graphics 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-core-image 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-core-location 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-core-text 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-encode 4.1.0 — MIT — https://github.com/madsmtm/objc2
- objc2-exception-helper 0.1.1 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-foundation 0.3.2 — MIT — https://github.com/madsmtm/objc2
- objc2-io-kit 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-io-surface 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-quartz-core 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-ui-kit 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-user-notifications 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- objc2-web-kit 0.3.2 — Zlib OR Apache-2.0 OR MIT — https://github.com/madsmtm/objc2
- once_cell 1.21.3 — MIT OR Apache-2.0 — https://github.com/matklad/once_cell
- open 5.3.3 — MIT — https://github.com/Byron/open-rs
- openssl-probe 0.2.1 — MIT OR Apache-2.0 — https://github.com/rustls/openssl-probe
- option-ext 0.2.0 — MPL-2.0 — https://github.com/soc/option-ext.git
- ordered-multimap 0.7.3 — MIT — https://github.com/sgodwincs/ordered-multimap-rs
- ordered-stream 0.2.0 — MIT OR Apache-2.0 — https://github.com/danieldg/ordered-stream
- os_info 3.14.0 — MIT — https://github.com/stanislav-tkach/os_info
- owned_ttf_parser 0.25.1 — Apache-2.0 — https://github.com/alexheretic/owned-ttf-parser
- pango 0.18.3 — MIT — https://github.com/gtk-rs/gtk-rs-core
- pango-sys 0.18.0 — MIT — https://github.com/gtk-rs/gtk-rs-core
- parking 2.2.1 — Apache-2.0 OR MIT — https://github.com/smol-rs/parking
- parking_lot 0.12.5 — MIT OR Apache-2.0 — https://github.com/Amanieu/parking_lot
- parking_lot_core 0.9.12 — MIT OR Apache-2.0 — https://github.com/Amanieu/parking_lot
- path-slash 0.2.1 — MIT — https://github.com/rhysd/path-slash
- pathdiff 0.2.3 — MIT/Apache-2.0 — https://github.com/Manishearth/pathdiff
- peeking_take_while 0.1.2 — Apache-2.0/MIT — https://github.com/fitzgen/peeking_take_while
- percent-encoding 2.3.2 — MIT OR Apache-2.0 — https://github.com/servo/rust-url/
- pest 2.8.6 — MIT OR Apache-2.0 — https://github.com/pest-parser/pest
- pest_derive 2.8.6 — MIT OR Apache-2.0 — https://github.com/pest-parser/pest
- pest_generator 2.8.6 — MIT OR Apache-2.0 — https://github.com/pest-parser/pest
- pest_meta 2.8.6 — MIT OR Apache-2.0 — https://github.com/pest-parser/pest
- petgraph 0.6.5 — MIT OR Apache-2.0 — https://github.com/petgraph/petgraph
- phf 0.10.1 — MIT — https://github.com/sfackler/rust-phf
- phf 0.11.3 — MIT — https://github.com/rust-phf/rust-phf
- phf 0.13.1 — MIT — https://github.com/rust-phf/rust-phf
- phf 0.8.0 — MIT — https://github.com/sfackler/rust-phf
- phf_codegen 0.11.3 — MIT — https://github.com/rust-phf/rust-phf
- phf_codegen 0.13.1 — MIT — https://github.com/rust-phf/rust-phf
- phf_codegen 0.8.0 — MIT — https://github.com/sfackler/rust-phf
- phf_generator 0.10.0 — MIT — https://github.com/sfackler/rust-phf
- phf_generator 0.11.3 — MIT — https://github.com/rust-phf/rust-phf
- phf_generator 0.13.1 — MIT — https://github.com/rust-phf/rust-phf
- phf_generator 0.8.0 — MIT — https://github.com/sfackler/rust-phf
- phf_macros 0.10.0 — MIT — https://github.com/sfackler/rust-phf
- phf_macros 0.13.1 — MIT — https://github.com/rust-phf/rust-phf
- phf_shared 0.10.0 — MIT — https://github.com/sfackler/rust-phf
- phf_shared 0.11.3 — MIT — https://github.com/rust-phf/rust-phf
- phf_shared 0.13.1 — MIT — https://github.com/rust-phf/rust-phf
- phf_shared 0.8.0 — MIT — https://github.com/sfackler/rust-phf
- pico-args 0.5.0 — MIT — https://github.com/RazrFalcon/pico-args
- pin-project-lite 0.2.16 — Apache-2.0 OR MIT — https://github.com/taiki-e/pin-project-lite
- pin-utils 0.1.0 — MIT OR Apache-2.0 — https://github.com/rust-lang-nursery/pin-utils
- piper 0.2.4 — MIT OR Apache-2.0 — https://github.com/smol-rs/piper
- pkg-config 0.3.32 — MIT OR Apache-2.0 — https://github.com/rust-lang/pkg-config-rs
- plist 1.8.0 — MIT — https://github.com/ebarnard/rust-plist/
- png 0.17.16 — MIT OR Apache-2.0 — https://github.com/image-rs/image-png
- png 0.18.1 — MIT OR Apache-2.0 — https://github.com/image-rs/image-png
- polling 3.11.0 — Apache-2.0 OR MIT — https://github.com/smol-rs/polling
- polycool 0.4.0 — MIT OR Apache-2.0 — https://github.com/linebender/kurbo
- potential_utf 0.1.4 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- powerfmt 0.2.0 — MIT OR Apache-2.0 — https://github.com/jhpratt/powerfmt
- ppv-lite86 0.2.21 — MIT OR Apache-2.0 — https://github.com/cryptocorrosion/cryptocorrosion
- precomputed-hash 0.1.1 — MIT — https://github.com/emilio/precomputed-hash
- prettyplease 0.2.37 — MIT OR Apache-2.0 — https://github.com/dtolnay/prettyplease
- proc-macro-crate 1.3.1 — MIT OR Apache-2.0 — https://github.com/bkchr/proc-macro-crate
- proc-macro-crate 2.0.2 — MIT OR Apache-2.0 — https://github.com/bkchr/proc-macro-crate
- proc-macro-crate 3.4.0 — MIT OR Apache-2.0 — https://github.com/bkchr/proc-macro-crate
- proc-macro-error 1.0.4 — MIT OR Apache-2.0 — https://gitlab.com/CreepySkeleton/proc-macro-error
- proc-macro-error-attr 1.0.4 — MIT OR Apache-2.0 — https://gitlab.com/CreepySkeleton/proc-macro-error
- proc-macro-hack 0.5.20+deprecated — MIT OR Apache-2.0 — https://github.com/dtolnay/proc-macro-hack
- proc-macro2 1.0.106 — MIT OR Apache-2.0 — https://github.com/dtolnay/proc-macro2
- psl-types 2.0.11 — MIT/Apache-2.0 — https://github.com/addr-rs/psl-types
- psp 0.1.0 — NOASSERTION — 
- ptr_meta 0.1.4 — MIT — https://github.com/djkoloski/ptr_meta
- ptr_meta_derive 0.1.4 — MIT — https://github.com/djkoloski/ptr_meta
- publicsuffix 2.3.0 — MIT/Apache-2.0 — https://github.com/rushmorem/publicsuffix
- pxfm 0.1.27 — BSD-3-Clause OR Apache-2.0 — https://github.com/awxkee/pxfm
- quick-xml 0.30.0 — MIT — https://github.com/tafia/quick-xml
- quick-xml 0.38.4 — MIT — https://github.com/tafia/quick-xml
- quinn 0.11.9 — MIT OR Apache-2.0 — https://github.com/quinn-rs/quinn
- quinn-proto 0.11.13 — MIT OR Apache-2.0 — https://github.com/quinn-rs/quinn
- quinn-udp 0.5.14 — MIT OR Apache-2.0 — https://github.com/quinn-rs/quinn
- quote 1.0.44 — MIT OR Apache-2.0 — https://github.com/dtolnay/quote
- r-efi 5.3.0 — MIT OR Apache-2.0 OR LGPL-2.1-or-later — https://github.com/r-efi/r-efi
- radium 0.7.0 — MIT — https://github.com/bitvecto-rs/radium
- rand 0.7.3 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand 0.8.5 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand 0.9.2 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand_chacha 0.2.2 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand_chacha 0.3.1 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand_chacha 0.9.0 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand_core 0.5.1 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand_core 0.6.4 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand_core 0.9.5 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- rand_hc 0.2.0 — MIT/Apache-2.0 — https://github.com/rust-random/rand
- rand_pcg 0.2.1 — MIT OR Apache-2.0 — https://github.com/rust-random/rand
- raw-window-handle 0.6.2 — MIT OR Apache-2.0 OR Zlib — https://github.com/rust-windowing/raw-window-handle
- redox_syscall 0.5.18 — MIT — https://gitlab.redox-os.org/redox-os/syscall
- redox_users 0.4.6 — MIT — https://gitlab.redox-os.org/redox-os/users
- redox_users 0.5.2 — MIT — https://gitlab.redox-os.org/redox-os/users
- ref-cast 1.0.25 — MIT OR Apache-2.0 — https://github.com/dtolnay/ref-cast
- ref-cast-impl 1.0.25 — MIT OR Apache-2.0 — https://github.com/dtolnay/ref-cast
- regex 1.12.3 — MIT OR Apache-2.0 — https://github.com/rust-lang/regex
- regex-automata 0.4.14 — MIT OR Apache-2.0 — https://github.com/rust-lang/regex
- regex-syntax 0.8.9 — MIT OR Apache-2.0 — https://github.com/rust-lang/regex
- rend 0.4.2 — MIT — https://github.com/djkoloski/rend
- reqwest 0.12.28 — MIT OR Apache-2.0 — https://github.com/seanmonstar/reqwest
- reqwest 0.13.1 — MIT OR Apache-2.0 — https://github.com/seanmonstar/reqwest
- resvg 0.47.0 — Apache-2.0 OR MIT — https://github.com/linebender/resvg
- rfd 0.16.0 — MIT — https://github.com/PolyMeilex/rfd
- rgb 0.8.53 — MIT — https://github.com/kornelski/rust-rgb
- ring 0.17.14 — Apache-2.0 AND ISC — https://github.com/briansmith/ring
- rkyv 0.7.46 — MIT — https://github.com/rkyv/rkyv
- rkyv_derive 0.7.46 — MIT — https://github.com/rkyv/rkyv
- roxmltree 0.20.0 — MIT OR Apache-2.0 — https://github.com/RazrFalcon/roxmltree
- roxmltree 0.21.1 — MIT OR Apache-2.0 — https://github.com/RazrFalcon/roxmltree
- rust-ini 0.21.3 — MIT — https://github.com/zonyitoo/rust-ini
- rust_decimal 1.40.0 — MIT — https://github.com/paupino/rust-decimal
- rustc-hash 1.1.0 — Apache-2.0/MIT — https://github.com/rust-lang-nursery/rustc-hash
- rustc-hash 2.1.1 — Apache-2.0 OR MIT — https://github.com/rust-lang/rustc-hash
- rustc_version 0.4.1 — MIT OR Apache-2.0 — https://github.com/djc/rustc-version-rs
- rustix 0.38.44 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/bytecodealliance/rustix
- rustix 1.1.3 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/bytecodealliance/rustix
- rustls 0.23.36 — Apache-2.0 OR ISC OR MIT — https://github.com/rustls/rustls
- rustls-native-certs 0.8.3 — Apache-2.0 OR ISC OR MIT — https://github.com/rustls/rustls-native-certs
- rustls-pki-types 1.14.0 — MIT OR Apache-2.0 — https://github.com/rustls/pki-types
- rustls-platform-verifier 0.6.2 — MIT OR Apache-2.0 — https://github.com/rustls/rustls-platform-verifier
- rustls-platform-verifier-android 0.1.1 — MIT OR Apache-2.0 — https://github.com/rustls/rustls-platform-verifier
- rustls-webpki 0.103.9 — ISC — https://github.com/rustls/webpki
- rustversion 1.0.22 — MIT OR Apache-2.0 — https://github.com/dtolnay/rustversion
- rustybuzz 0.20.1 — MIT — https://github.com/harfbuzz/rustybuzz
- ryu 1.0.22 — Apache-2.0 OR BSL-1.0 — https://github.com/dtolnay/ryu
- same-file 1.0.6 — Unlicense/MIT — https://github.com/BurntSushi/same-file
- schannel 0.1.28 — MIT — https://github.com/steffengy/schannel-rs
- schemars 0.8.22 — MIT — https://github.com/GREsau/schemars
- schemars 0.9.0 — MIT — https://github.com/GREsau/schemars
- schemars 1.2.1 — MIT — https://github.com/GREsau/schemars
- schemars_derive 0.8.22 — MIT — https://github.com/GREsau/schemars
- scopeguard 1.2.0 — MIT OR Apache-2.0 — https://github.com/bluss/scopeguard
- seahash 4.1.0 — MIT — https://gitlab.redox-os.org/redox-os/seahash
- security-framework 3.5.1 — MIT OR Apache-2.0 — https://github.com/kornelski/rust-security-framework
- security-framework-sys 2.15.0 — MIT OR Apache-2.0 — https://github.com/kornelski/rust-security-framework
- selectors 0.24.0 — MPL-2.0 — https://github.com/servo/servo
- selectors 0.38.0 — MPL-2.0 — https://github.com/servo/stylo
- semver 1.0.28 — MIT OR Apache-2.0 — https://github.com/dtolnay/semver
- serde 1.0.228 — MIT OR Apache-2.0 — https://github.com/serde-rs/serde
- serde-inline-default 1.0.1 — MIT OR Apache-2.0 — https://github.com/bytedream/serde-inline-default
- serde-untagged 0.1.9 — MIT OR Apache-2.0 — https://github.com/dtolnay/serde-untagged
- serde_core 1.0.228 — MIT OR Apache-2.0 — https://github.com/serde-rs/serde
- serde_derive 1.0.228 — MIT OR Apache-2.0 — https://github.com/serde-rs/serde
- serde_derive_internals 0.29.1 — MIT OR Apache-2.0 — https://github.com/serde-rs/serde
- serde_json 1.0.149 — MIT OR Apache-2.0 — https://github.com/serde-rs/json
- serde_repr 0.1.20 — MIT OR Apache-2.0 — https://github.com/dtolnay/serde-repr
- serde_spanned 0.6.9 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- serde_spanned 1.1.1 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- serde_urlencoded 0.7.1 — MIT/Apache-2.0 — https://github.com/nox/serde_urlencoded
- serde_with 3.16.1 — MIT OR Apache-2.0 — https://github.com/jonasbb/serde_with/
- serde_with_macros 3.16.1 — MIT OR Apache-2.0 — https://github.com/jonasbb/serde_with/
- serialize-to-javascript 0.1.2 — MIT OR Apache-2.0 — https://github.com/chippers/serialize-to-javascript
- serialize-to-javascript-impl 0.1.2 — MIT OR Apache-2.0 — https://github.com/chippers/serialize-to-javascript
- servo-fontconfig 0.5.1 — MIT / Apache-2.0 — https://github.com/servo/rust-fontconfig/
- servo-fontconfig-sys 5.1.0 — MIT — https://github.com/servo/libfontconfig/
- servo_arc 0.2.0 — MIT OR Apache-2.0 — https://github.com/servo/servo
- servo_arc 0.4.3 — MIT OR Apache-2.0 — https://github.com/servo/stylo
- sha1 0.10.6 — MIT OR Apache-2.0 — https://github.com/RustCrypto/hashes
- sha2 0.10.9 — MIT OR Apache-2.0 — https://github.com/RustCrypto/hashes
- shlex 1.3.0 — MIT OR Apache-2.0 — https://github.com/comex/rust-shlex
- signal-hook-registry 1.4.8 — MIT OR Apache-2.0 — https://github.com/vorner/signal-hook
- simd-adler32 0.3.8 — MIT — https://github.com/mcountryman/simd-adler32
- simdutf8 0.1.5 — MIT OR Apache-2.0 — https://github.com/rusticstuff/simdutf8
- simplecss 0.2.2 — Apache-2.0 OR MIT — https://github.com/linebender/simplecss
- siphasher 0.3.11 — MIT/Apache-2.0 — https://github.com/jedisct1/rust-siphash
- siphasher 1.0.2 — MIT/Apache-2.0 — https://github.com/jedisct1/rust-siphash
- slab 0.4.12 — MIT — https://github.com/tokio-rs/slab
- slotmap 1.1.1 — Zlib — https://github.com/orlp/slotmap
- smallvec 1.15.1 — MIT OR Apache-2.0 — https://github.com/servo/rust-smallvec
- socket2 0.6.2 — MIT OR Apache-2.0 — https://github.com/rust-lang/socket2
- softbuffer 0.4.8 — MIT OR Apache-2.0 — https://github.com/rust-windowing/softbuffer
- soup3 0.5.0 — MIT — https://gitlab.gnome.org/World/Rust/soup3-rs
- soup3-sys 0.5.0 — MIT — https://gitlab.gnome.org/World/Rust/soup3-rs
- stable_deref_trait 1.2.1 — MIT OR Apache-2.0 — https://github.com/storyyeller/stable_deref_trait
- streamdeck-strip-render 0.1.0 — NOASSERTION — 
- strict-num 0.1.1 — MIT — https://github.com/RazrFalcon/strict-num
- string_cache 0.8.9 — MIT OR Apache-2.0 — https://github.com/servo/string-cache
- string_cache 0.9.0 — MIT OR Apache-2.0 — https://github.com/servo/string-cache
- string_cache_codegen 0.5.4 — MIT OR Apache-2.0 — https://github.com/servo/string-cache
- string_cache_codegen 0.6.1 — MIT OR Apache-2.0 — https://github.com/servo/string-cache
- strip-ansi-escapes 0.2.1 — Apache-2.0/MIT — https://github.com/luser/strip-ansi-escapes
- strsim 0.11.1 — MIT — https://github.com/rapidfuzz/strsim-rs
- subtle 2.6.1 — BSD-3-Clause — https://github.com/dalek-cryptography/subtle
- svgtypes 0.16.1 — Apache-2.0 OR MIT — https://github.com/linebender/svgtypes
- swift-rs 1.0.8 — MIT OR Apache-2.0 — https://github.com/Brendonovich/swift-rs
- syn 1.0.109 — MIT OR Apache-2.0 — https://github.com/dtolnay/syn
- syn 2.0.114 — MIT OR Apache-2.0 — https://github.com/dtolnay/syn
- sync_wrapper 1.0.2 — Apache-2.0 — https://github.com/Actyx/sync_wrapper
- synstructure 0.13.2 — MIT — https://github.com/mystor/synstructure
- sysinfo 0.38.0 — MIT — https://github.com/GuillaumeGomez/sysinfo
- system-configuration 0.7.0 — MIT OR Apache-2.0 — https://github.com/mullvad/system-configuration-rs
- system-configuration-sys 0.6.0 — MIT OR Apache-2.0 — https://github.com/mullvad/system-configuration-rs
- system-deps 6.2.2 — MIT OR Apache-2.0 — https://github.com/gdesmott/system-deps
- tao 0.37.1 — Apache-2.0 — https://github.com/tauri-apps/tao
- tao-macros 0.1.4 — MIT OR Apache-2.0 — https://github.com/tauri-apps/tao
- tap 1.0.1 — MIT — https://github.com/myrrlyn/tap
- target-lexicon 0.12.16 — Apache-2.0 WITH LLVM-exception — https://github.com/bytecodealliance/target-lexicon
- tauri 2.12.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-build 2.7.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-codegen 2.7.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-macros 2.7.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-plugin 2.5.3 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-plugin-autostart 2.5.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/plugins-workspace
- tauri-plugin-cors-fetch 4.2.0 — MIT — https://github.com/idootop/tauri-plugin-cors-fetch
- tauri-plugin-deep-link 2.4.7 — Apache-2.0 OR MIT — https://github.com/tauri-apps/plugins-workspace
- tauri-plugin-dialog 2.6.0 — Apache-2.0 OR MIT — https://github.com/tauri-apps/plugins-workspace
- tauri-plugin-fs 2.4.5 — Apache-2.0 OR MIT — https://github.com/tauri-apps/plugins-workspace
- tauri-plugin-log 2.8.0 — Apache-2.0 OR MIT — https://github.com/tauri-apps/plugins-workspace
- tauri-plugin-single-instance 2.4.0 — Apache-2.0 OR MIT — https://github.com/tauri-apps/plugins-workspace
- tauri-runtime 2.12.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-runtime-wry 2.12.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-utils 2.10.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri
- tauri-winres 0.3.5 — MIT — https://github.com/tauri-apps/winres
- tempfile 3.24.0 — MIT OR Apache-2.0 — https://github.com/Stebalien/tempfile
- tendril 0.4.3 — MIT/Apache-2.0 — https://github.com/servo/tendril
- tendril 0.5.1 — MIT OR Apache-2.0 — https://github.com/servo/html5ever
- thiserror 1.0.69 — MIT OR Apache-2.0 — https://github.com/dtolnay/thiserror
- thiserror 2.0.18 — MIT OR Apache-2.0 — https://github.com/dtolnay/thiserror
- thiserror-impl 1.0.69 — MIT OR Apache-2.0 — https://github.com/dtolnay/thiserror
- thiserror-impl 2.0.18 — MIT OR Apache-2.0 — https://github.com/dtolnay/thiserror
- time 0.3.47 — MIT OR Apache-2.0 — https://github.com/time-rs/time
- time-core 0.1.8 — MIT OR Apache-2.0 — https://github.com/time-rs/time
- time-macros 0.2.27 — MIT OR Apache-2.0 — https://github.com/time-rs/time
- tiny-keccak 2.0.2 — CC0-1.0 — 
- tiny-skia 0.12.0 — BSD-3-Clause — https://github.com/linebender/tiny-skia
- tiny-skia-path 0.12.0 — BSD-3-Clause — https://github.com/linebender/tiny-skia/tree/master/path
- tiny_http 0.12.0 — MIT OR Apache-2.0 — https://github.com/tiny-http/tiny-http
- tinystr 0.8.2 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- tinyvec 1.10.0 — Zlib OR Apache-2.0 OR MIT — https://github.com/Lokathor/tinyvec
- tinyvec_macros 0.1.1 — MIT OR Apache-2.0 OR Zlib — https://github.com/Soveu/tinyvec_macros
- tokio 1.49.0 — MIT — https://github.com/tokio-rs/tokio
- tokio-macros 2.6.0 — MIT — https://github.com/tokio-rs/tokio
- tokio-rustls 0.26.4 — MIT OR Apache-2.0 — https://github.com/rustls/tokio-rustls
- tokio-tungstenite 0.28.0 — MIT — https://github.com/snapview/tokio-tungstenite
- tokio-util 0.7.18 — MIT — https://github.com/tokio-rs/tokio
- toml 0.8.2 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml 0.9.11+spec-1.1.0 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml 1.1.6+spec-1.1.0 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_datetime 0.6.3 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_datetime 0.7.5+spec-1.1.0 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_datetime 1.1.1+spec-1.1.0 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_edit 0.19.15 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_edit 0.20.2 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_edit 0.23.10+spec-1.0.0 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_parser 1.1.3+spec-1.1.0 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- toml_writer 1.1.2+spec-1.1.0 — MIT OR Apache-2.0 — https://github.com/toml-rs/toml
- tower 0.5.3 — MIT — https://github.com/tower-rs/tower
- tower-http 0.6.8 — MIT — https://github.com/tower-rs/tower-http
- tower-layer 0.3.3 — MIT — https://github.com/tower-rs/tower
- tower-service 0.3.3 — MIT — https://github.com/tower-rs/tower
- tracing 0.1.44 — MIT — https://github.com/tokio-rs/tracing
- tracing-attributes 0.1.31 — MIT — https://github.com/tokio-rs/tracing
- tracing-core 0.1.36 — MIT — https://github.com/tokio-rs/tracing
- tray-icon 0.25.1 — MIT OR Apache-2.0 — https://github.com/tauri-apps/tray-icon
- try-lock 0.2.5 — MIT — https://github.com/seanmonstar/try-lock
- ttf-parser 0.25.1 — MIT OR Apache-2.0 — https://github.com/harfbuzz/ttf-parser
- tungstenite 0.28.0 — MIT OR Apache-2.0 — https://github.com/snapview/tungstenite-rs
- typeid 1.0.3 — MIT OR Apache-2.0 — https://github.com/dtolnay/typeid
- typenum 1.19.0 — MIT OR Apache-2.0 — https://github.com/paholg/typenum
- ucd-trie 0.1.7 — MIT OR Apache-2.0 — https://github.com/BurntSushi/ucd-generate
- uds_windows 1.1.0 — MIT — https://github.com/haraldh/rust_uds_windows
- unicode-bidi 0.3.18 — MIT OR Apache-2.0 — https://github.com/servo/unicode-bidi
- unicode-bidi-mirroring 0.4.0 — MIT/Apache-2.0 — https://github.com/RazrFalcon/unicode-bidi-mirroring
- unicode-ccc 0.4.0 — MIT/Apache-2.0 — https://github.com/RazrFalcon/unicode-ccc
- unicode-ident 1.0.22 — (MIT OR Apache-2.0) AND Unicode-3.0 — https://github.com/dtolnay/unicode-ident
- unicode-properties 0.1.4 — MIT/Apache-2.0 — https://github.com/unicode-rs/unicode-properties
- unicode-script 0.5.8 — MIT OR Apache-2.0 — https://github.com/unicode-rs/unicode-script
- unicode-segmentation 1.12.0 — MIT OR Apache-2.0 — https://github.com/unicode-rs/unicode-segmentation
- unicode-vo 0.1.0 — MIT/Apache-2.0 — https://github.com/RazrFalcon/unicode-vo
- untrusted 0.9.0 — ISC — https://github.com/briansmith/untrusted
- url 2.5.8 — MIT OR Apache-2.0 — https://github.com/servo/rust-url
- urlencoding 2.1.3 — MIT — https://github.com/kornelski/rust_urlencoding
- urlpattern 0.6.0 — MIT — https://github.com/denoland/rust-urlpattern
- usvg 0.47.0 — Apache-2.0 OR MIT — https://github.com/linebender/resvg
- utf-8 0.7.6 — MIT OR Apache-2.0 — https://github.com/SimonSapin/rust-utf8
- utf8-width 0.1.8 — MIT — https://github.com/magiclen/utf8-width
- utf8_iter 1.0.4 — Apache-2.0 OR MIT — https://github.com/hsivonen/utf8_iter
- uuid 1.20.0 — Apache-2.0 OR MIT — https://github.com/uuid-rs/uuid
- value-bag 1.12.0 — Apache-2.0 OR MIT — https://github.com/sval-rs/value-bag
- vcpkg 0.2.15 — MIT/Apache-2.0 — https://github.com/mcgoo/vcpkg-rs
- version-compare 0.2.1 — MIT — https://gitlab.com/timvisee/version-compare
- version_check 0.9.5 — MIT/Apache-2.0 — https://github.com/SergioBenitez/version_check
- vswhom 0.1.0 — MIT — https://github.com/nabijaczleweli/vswhom.rs
- vswhom-sys 0.1.3 — MIT — https://github.com/nabijaczleweli/vswhom-sys.rs
- vte 0.14.1 — Apache-2.0 OR MIT — https://github.com/alacritty/vte
- walkdir 2.5.0 — Unlicense/MIT — https://github.com/BurntSushi/walkdir
- want 0.3.1 — MIT — https://github.com/seanmonstar/want
- wasi 0.11.1+wasi-snapshot-preview1 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/bytecodealliance/wasi
- wasi 0.9.0+wasi-snapshot-preview1 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/bytecodealliance/wasi
- wasip2 1.0.2+wasi-0.2.9 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/bytecodealliance/wasi-rs
- wasm-bindgen 0.2.108 — MIT OR Apache-2.0 — https://github.com/wasm-bindgen/wasm-bindgen
- wasm-bindgen-futures 0.4.58 — MIT OR Apache-2.0 — https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/futures
- wasm-bindgen-macro 0.2.108 — MIT OR Apache-2.0 — https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro
- wasm-bindgen-macro-support 0.2.108 — MIT OR Apache-2.0 — https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro-support
- wasm-bindgen-shared 0.2.108 — MIT OR Apache-2.0 — https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared
- wasm-streams 0.4.2 — MIT OR Apache-2.0 — https://github.com/MattiasBuelens/wasm-streams/
- web-sys 0.3.85 — MIT OR Apache-2.0 — https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/web-sys
- web-time 1.1.0 — MIT OR Apache-2.0 — https://github.com/daxpedda/web-time
- web_atoms 0.2.6 — MIT OR Apache-2.0 — https://github.com/servo/html5ever
- webkit2gtk 2.0.2 — MIT — https://github.com/tauri-apps/webkit2gtk-rs
- webkit2gtk-sys 2.0.2 — MIT — https://github.com/tauri-apps/webkit2gtk-rs
- webpki-root-certs 1.0.6 — CDLA-Permissive-2.0 — https://github.com/rustls/webpki-roots
- webpki-roots 1.0.6 — CDLA-Permissive-2.0 — https://github.com/rustls/webpki-roots
- webview2-com 0.39.1 — MIT — https://github.com/wravery/webview2-rs
- webview2-com-macros 0.8.1 — MIT — https://github.com/wravery/webview2-rs
- webview2-com-sys 0.39.1 — MIT — https://github.com/wravery/webview2-rs
- which 4.4.2 — MIT — https://github.com/harryfei/which-rs.git
- winapi 0.3.9 — MIT/Apache-2.0 — https://github.com/retep998/winapi-rs
- winapi-i686-pc-windows-gnu 0.4.0 — MIT/Apache-2.0 — https://github.com/retep998/winapi-rs
- winapi-util 0.1.11 — Unlicense OR MIT — https://github.com/BurntSushi/winapi-util
- winapi-x86_64-pc-windows-gnu 0.4.0 — MIT/Apache-2.0 — https://github.com/retep998/winapi-rs
- window-vibrancy 0.8.1 — Apache-2.0 OR MIT — https://github.com/tauri-apps/tauri-plugin-vibrancy
- windows 0.48.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows 0.61.3 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows 0.62.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-collections 0.2.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-collections 0.3.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-core 0.61.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-core 0.62.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-future 0.2.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-future 0.3.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-implement 0.60.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-interface 0.59.3 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-link 0.1.3 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-link 0.2.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-numerics 0.2.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-numerics 0.3.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-registry 0.5.3 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-registry 0.6.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-result 0.3.4 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-result 0.4.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-strings 0.4.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-strings 0.5.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-sys 0.45.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-sys 0.48.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-sys 0.52.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-sys 0.59.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-sys 0.60.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-sys 0.61.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-targets 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-targets 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-targets 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-targets 0.53.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-threading 0.1.0 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-threading 0.2.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows-version 0.1.7 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_gnullvm 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_gnullvm 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_gnullvm 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_gnullvm 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_msvc 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_msvc 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_msvc 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_aarch64_msvc 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_gnu 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_gnu 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_gnu 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_gnu 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_gnullvm 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_gnullvm 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_msvc 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_msvc 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_msvc 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_i686_msvc 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnu 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnu 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnu 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnu 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnullvm 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnullvm 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnullvm 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_gnullvm 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_msvc 0.42.2 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_msvc 0.48.5 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_msvc 0.52.6 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- windows_x86_64_msvc 0.53.1 — MIT OR Apache-2.0 — https://github.com/microsoft/windows-rs
- winnow 0.5.40 — MIT — https://github.com/winnow-rs/winnow
- winnow 0.7.14 — MIT — https://github.com/winnow-rs/winnow
- winnow 1.0.4 — MIT — https://github.com/winnow-rs/winnow
- winreg 0.10.1 — MIT — https://github.com/gentoo90/winreg-rs
- winreg 0.55.0 — MIT — https://github.com/gentoo90/winreg-rs
- wit-bindgen 0.51.0 — Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT — https://github.com/bytecodealliance/wit-bindgen
- writeable 0.6.2 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- wry 0.57.0 — Apache-2.0 OR MIT — https://github.com/tauri-apps/wry
- wyz 0.5.1 — MIT — https://github.com/myrrlyn/wyz
- x11 2.21.0 — MIT — https://github.com/AltF02/x11-rs.git
- x11-dl 2.21.0 — MIT — https://github.com/AltF02/x11-rs.git
- xcb 1.7.0 — MIT — https://github.com/rust-x-bindings/rust-xcb
- xmlwriter 0.1.0 — MIT — https://github.com/RazrFalcon/xmlwriter
- yoke 0.8.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- yoke-derive 0.8.1 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- zbus 5.13.2 — MIT — https://github.com/z-galaxy/zbus/
- zbus_macros 5.13.2 — MIT — https://github.com/z-galaxy/zbus/
- zbus_names 4.3.1 — MIT — https://github.com/z-galaxy/zbus/
- zerocopy 0.8.39 — BSD-2-Clause OR Apache-2.0 OR MIT — https://github.com/google/zerocopy
- zerocopy-derive 0.8.39 — BSD-2-Clause OR Apache-2.0 OR MIT — https://github.com/google/zerocopy
- zerofrom 0.1.6 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- zerofrom-derive 0.1.6 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- zeroize 1.8.2 — Apache-2.0 OR MIT — https://github.com/RustCrypto/utils
- zerotrie 0.2.3 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- zerovec 0.11.5 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- zerovec-derive 0.11.2 — Unicode-3.0 — https://github.com/unicode-org/icu4x
- zip 6.0.0 — MIT — https://github.com/zip-rs/zip2.git
- zlib-rs 0.6.0 — Zlib — https://github.com/trifectatechfoundation/zlib-rs
- zmij 1.0.19 — MIT — https://github.com/dtolnay/zmij
- zopfli 0.8.3 — Apache-2.0 — https://github.com/zopfli-rs/zopfli
- zstd 0.13.3 — MIT — https://github.com/gyscos/zstd-rs
- zstd-safe 7.2.4 — MIT OR Apache-2.0 — https://github.com/gyscos/zstd-rs
- zstd-sys 2.0.16+zstd.1.5.7 — MIT/Apache-2.0 — https://github.com/gyscos/zstd-rs
- zune-core 0.5.1 — MIT OR Apache-2.0 OR Zlib — https://github.com/etemesi254/zune-image
- zune-jpeg 0.5.12 — MIT OR Apache-2.0 OR Zlib — https://github.com/etemesi254/zune-image/tree/dev/crates/zune-jpeg
- zvariant 5.9.2 — MIT — https://github.com/z-galaxy/zbus/
- zvariant_derive 5.9.2 — MIT — https://github.com/z-galaxy/zbus/
- zvariant_utils 3.3.0 — MIT — https://github.com/z-galaxy/zbus/
