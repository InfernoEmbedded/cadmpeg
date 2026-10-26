# SolidWorks `.sldprt`: Open Items

This document lists the parts of the SolidWorks `.sldprt` format that we do not know. The specification `sldprt.md` gives the parts that we know.

Each item has these parts:

- **Question** — what we must find.
- **Known** — what the specification gives now.
- **Need** — why we must find the answer.
- **Conflict** — a disagreement between two documents, or between a document and the decoder. An item with this part needs a decision.
- **Note** — a defect in the item or in the specification.

When an item is resolved, delete it in the same change that writes the answer into the specification. Do not keep a Resolved part.

Each item has an identifier. Use the identifier in commit messages and in code comments.

This document uses ASD-STE100 Simplified Technical English. Record names, field names, and token values are technical names. They keep their source spelling.

## 1. Body classification

### BC-04. Duplicate topology record identities

**Question.** Which occurrence is authoritative when one stream contains multiple valid topology records with the same attribute but different references or coordinates?

**Known.** Topology records are keyed by stream-local attribute. A complete short-index FACE includes five pointers after its sense. Complete EDGE and VERTEX bodies include their trailing owner pointers. POINT has exactly four pointers before its coordinate vector. Equal projected fields do not establish equality of complete record bytes. The specification does not define a duplicate-record rule or a native precedence field.

**Need.** We must select the authoritative topology record to avoid silently changing body connectivity or vertex positions when a duplicate identity is present.

### BC-08. Complete identity in single-face component selections

**Question.** Which single-face component-selection field, if any, supplies the complete ordered tail of a persistent face identity?

**Known.** Complete `FACE_ID_2001` and DisplayFace identities include tail length and ordered values. A source/local pair can name several distinct complete identities. A single-face selector carrying only that pair requires one distinct target. Generated-hole surface references expand the pair to a set and do not select one persistent face. A component selection has an ordered path of type signatures and optional local identifiers; path length and earlier local identifiers do not establish a persistent-identity tail.

**Need.** An explicit native field relation must distinguish a complete selector from a prefix-only selector before a colliding source/local pair can select one face.

## 2. Geometry carriers

### GC-02. Missing intersection witnesses

**Question.** Where are the authoritative chart and terminator replacements when an intersection composite references an absent witness record?

**Known.** `sldprt.md` §7.2 "Curve carriers: an edge's `00 10.refs[3]` can point to a `00 86` B-spline/list curve carrier," through `sldprt.md` §7.3 "The chart is a solved cache" define both intersection carriers and the three witness record families. The referenced terminators select the unique chart stride with the least endpoint displacement and replace the approximate chart endpoints. A complete width-4 support-UV record supplies the two solved pcurve caches. An absent or structurally inconsistent optional support-UV record does not invalidate the curve. Tolerance-bounded inversion constructs its pcurves on analytic and positive-weight NURBS supports. Consecutive inverses select one continuous parameter branch, the terminators supply the exact endpoint parameters, and analytic derivative bounds or rational Bézier residual control hulls certify the complete mapped segments against the chart tolerance.

**Need.** We must locate an authoritative replacement when the referenced chart or terminator record is absent.

### GC-03. Surface-owned edge curve attributes

**Question.** What grammar and role do untyped edge curve attributes have when a surface record owns them instead of a curve carrier?

**Known.** `sldprt.md` §7.1 "Stream-scope" defines `00 86` as a B-spline or list curve carrier. `sldprt.md` §7.2 "Curve carriers: an edge's `00 10.refs[3]` can point to a `00 86` B-spline/list curve carrier," defines the intersection curve carriers. An edge selects its support curve through `00 10.refs[3]`.

**Need.** We must know the grammar to construct the edge curve.

### GC-04. Offset-surface carriers

**Question.** Which carrier and field semantics identify an offset surface, including the discriminator and flag in its native record?

**Known.** `sldprt.md` §7.4 defines the supported offset carrier and its signed normal-offset construction over an analytic, B-spline, or procedural support surface.

**Need.** We must know the carrier grammar and field semantics to construct the exact offset surface and preserve its orientation.

### GC-05. Variable-radius law and result parameterization

**Question.** How does a complete variable-radius law, including intermediate controls, map to the parameterization and individual faces of every result patch?

**Known.** The design law and evaluated result have separate carriers. An owned generated-surface identity joins a typed FACE through its complete `FACE_ID_2001` identity; the FACE's surface reference selects the stored NURBS carrier. Resolve the attribute definition's identifier, not an adjacent KEY name. Two independently checked joins retain ordered endpoint radii of 5–50 mm and 50–4 mm, and four rational NURBS endpoint sections reproduce those radii. The existing reader retains both laws and the generated identities. These bounded observations do not establish a corpus census or a general interpolation rule.

**Need.** Prove intermediate-control parameterization, complete selector-tail and collision behavior, and the explicit per-feature result-face association across arbitrary patches. Missing or nonunique complete identities must remain unresolved; geometric resemblance and history order cannot choose a result.

### GC-08. Duplicate geometry carrier identities

**Question.** Which occurrence is authoritative when one stream contains multiple valid geometry carriers with the same stream-local attribute?

**Known.** Geometry carriers are indexed by stream-local attribute. Analytic decode refuses a uniquely parsed carrier whose header framing disagrees with the Partition/Deltas role selected from the stream header. The role filter cannot recover a record rejected by the role-unknown parser, and the writer retains that conservative role-unknown path. Stream role does not prove chronology or duplicate authority.

**Need.** We must select the authoritative carrier without silently changing geometry when a duplicate record is present.

### GC-09. Conflicting surface carrier families

**Question.** Which surface carrier family owns a face when analytic, offset, blend, or sweep candidates are all structurally valid for the same surface attribute?

**Known.** `sldprt.md` §7 defines analytic and procedural surface carriers, but does not define a conflict rule between valid carrier families.

**Need.** We must define the native family discriminator or ownership relation that selects one exact face support when several carrier families are structurally valid.

## 3. Container metadata

### CM-01. Cache-cell prefix

**Question.** Which producer rule generates a new cache cell's opaque prefix values when no source bytes are available?

**Known.** The prefix occupies the `N=L/S` bytes between an independently anchored cell start and its marker. Equal section names, displacements, complete fixed headers, and unique target blocks do not determine equal prefix values. Retained cells preserve these bytes without assigning a producer meaning.

**Need.** A producer-side relation must establish a regeneration rule before a source-less writer can synthesize this field.

### CM-02. Cache-cell fill

**Question.** Which producer rule generates a new cache cell's opaque trailing values when no source bytes are available?

**Known.** The trailing field extends from the encoded name's end to `cell_start+S`. Equal section names, displacements, complete fixed headers, and unique target blocks do not determine equal trailing values. Zero and `ff` are opaque byte values, not structural rejection conditions. Retained cells preserve the complete range.

**Need.** A producer-side relation must establish a regeneration rule before a source-less writer can synthesize this field.

### CM-04. Opaque tail residual generation

**Question.** How does the producer generate the opaque prefix and residual bytes in the named tail-padding and anonymous footer cells?

**Known.** `sldprt.md` §1.4 defines the named-grid scales, footer scales, exact cell extents, EOCD count, and both central-directory locator equations. Bytes outside those fields remain opaque and are retained exactly; they do not select directory members.

**Need.** An independent producer oracle that explains or generates those residual bytes for newly constructed cells.

**Conflict.** None.

**Note.** Parsed grid geometry and EOCD fields do not establish a regeneration rule for opaque residuals.

### CM-07. Transformed enclosure opaque prologue

**Question.** How does the producer generate the 24 opaque bytes in a `moTransRefPlaneData_c` serializer prologue?

**Known.** `sldprt.md` §8 defines its fixed 25-byte prologue, presence byte, and optional ten-value enclosure. The stored center, corners, and radius do not define the opaque bytes.

**Need.** An independent producer rule or controlled native edit must determine these bytes for a newly authored record.

**Conflict.** None.

**Note.** Retained prologue bytes can be preserved without assigning a generation rule.

### CM-10. Complete effective-schema stream cursor

**Question.** Which base schemas and first-use field sequences establish every XT node boundary in unframed carriers?

**Known.** `sldprt.md` §3.1.1 defines the terminal node and the direct `ThirdPty/WELDMENT_STORAGE` length prefix. The chained wrapper section length bounds its concatenated frame outputs. Header-shaped signatures and terminal-shaped field bytes do not define an intrinsic boundary. The complete base-schema node cursor is separate from outer-extent and endpoint-wire checks.

**Need.** Complete selected base-schema field tables and Source-backed traversal of each effective field edit, fixed field, and variable count must establish the node cursor before intrinsic endpoints can be admitted without a carrier extent.

**Conflict.** None.

**Note.** Outer framing does not establish physical occurrence policy or permit salvage of an invalid CFB extent.

### CM-14. Configuration identity carriers and update state

**Question.** Which carrier controls membership when Keywords `id` differs from manifest `swID`, or a `NeedsUpdate="YES"` row has a decoded partition?

**Known.** `sldprt.md` §2 defines the complete Keywords-name, manifest-row, referenced-model, and decimal `swID` join for current inactive and active snapshots. Missing partitions leave membership unresolved; decoded empty partitions establish empty membership. Keywords `id` and `swID` coincide in the available witnesses, so their covariance does not establish interchangeable authority. For configurations without a valid explicit index, the reader requires the complete manifest join and `NeedsUpdate="NO"`; it abstains on a conflicting explicit `SourceIndex` or an update-required row with a present partition. These abstentions do not establish producer precedence in the unobserved cases.

**Need.** A producer save or export must distinguish unequal Keywords `id` and manifest `swID`, and separately distinguish a `YES` row with a present current partition. A native dual-carrier witness must test explicit `SourceIndex` disagreement with the full manifest join.

## 4. Auxiliary lanes

### AL-01. DisplayLists face ranges

**Question.** How does a B-rep face attribute select its triangle range in a DisplayLists block?

**Known.** `sldprt.md` §7.3 "**`00 28` chart**" defines the DisplayLists descriptor table, strip lengths, and triangle-count relations. `sldprt.md` §4 "Primary ownership chain:" defines the B-rep face topology. `sldprt.md` §8 defines the complete persistent identity path carried by matching B-rep attributes and DisplayLists references, as well as unambiguous ownership from incidence with one analytic or NURBS face support and from complete bounded planar, cylindrical, and conical trims. Procedural, polygonal, opaque, and non-exact NURBS supports, other coincident trims, and tables without a complete matching identity do not supply the stored face-range mapping.

**Need.** We must know the stored mapping to attach tables whose support is ambiguous or unsupported, including non-exact NURBS caches, and carry no complete matching persistent identity.

### AL-03. DisplayLists renderer flag meanings

**Question.** Which renderer states do the stored `B` and `C` flags identify?

**Known.** `sldprt.md` §8 defines the four supported extended header words, their Boolean metadata triples, the exact stored-normal classification, and the optional scalar/unit-vector tuple. The independent metadata word does not contribute to the header word.

**Need.** We must know the renderer meanings of `B` and `C` and the rule that selects their values when no retained metadata supplies them.

### AL-05. Retained and direct face appearance paths

**Question.** Which complete persistent-path relation selects retained feature faces and direct face appearances?

**Known.** `sldprt.md` §8 defines native display-state selection, three complete part-default class chains, body identity through `ATOM_ID_2001`, and generated-face feature scope through current `FACE_ID_2001` source and timestamp values. Four-word face values identify a feature-scoped owner set without a local face selector. A `moCompFace_c` object can carry a complete multi-component path. A terminal source, timestamp, and local triple can match more than one current face or omit another affected current face. A bare definition supplies no ownership. Unresolved higher-rank paths do not supply lower-rank appearance fallbacks.

**Need.** The complete path-to-current-face binding for retained or modified feature faces and direct face assignments, and a complete holderless part-default owner relation. A unique terminal triple in one current identity set does not establish a general path-truncation rule.

## 5. Design intent

### DI-01. Other optional feature-manager node identities

**Question.** What native field identifies the role of each optional classless feature-manager node that does not satisfy a defined layout-scoped identity?

**Known.** `sldprt.md` §2 "An `moCurvePattern_c` feature-input object is immediately preceded by its seed feature object" through `sldprt.md` §2 "An `moLPattern_c` interval without a line-reference record carries each displayed translation" identify the annotations container, principal planes, model origin, lights-and-cameras container, ambient and directional lights, sheet-metal node, and exploded-views container. The identities depend on a complete native-class roster. Other source identifiers are allocation positions and are not role codes. `sldprt.md` §2 "A classless Keywords `Feature` whose `Type` token is `EquationDriven` is the equation" identifies the equation container by its operation-family token, which is a role code and needs neither a native class nor a reserved source identifier. `sldprt.md` §8 defines the explicit DisplayLists scene-object source binding. Anonymous scene-object counts do not identify Keywords records.

**Need.** We must distinguish the remaining binders, comments, body folders, materials, notes, sensors, favorites, history, selection sets, and markups.

### DI-02. Relation-manager settings boundary

**Question.** What complete relation-list and intervening-text grammar identifies the post-relation settings boundary for each supported relation-manager layout?

**Known.** `sldprt.md` §2 "The equation angular-unit mode is the byte" defines the settings-relative field and native Degrees/Radians encoding. The relation list contains a declared count of variable-length objects. Nested expressions and drafting-standard strings change the displacement of the settings prefix from the manager declaration. Display-angle units do not establish the equation mode.

**Need.** A counted parser must consume each complete relation and its nested expression objects, then frame any intervening text before it returns the settings prefix. It must distinguish unsupported manager layouts and missing configuration copies. Numeric signatures inside unparsed relation bytes do not establish that boundary.

**Conflict.** None.

**Note.** A field-relative layout does not define the preceding variable-length grammar. A mode value does not prove that a relation evaluator applied it to a bare angular operand.

### DI-03. Calculated default-property caches

**Question.** Which native state establishes that a saved calculated default-property cache is current for the requested property operand?

**Known.** `sldprt.md` §8.1 "The document default-property namespace is owned by" defines the complete carrier, FMTID, PID and alias identity. It distinguishes saved values from formula companions and live calculation. Configuration-qualified property namespace lookup is DI-04.

**Need.** A source-level freshness rule and an independently validated cache-owner relation before a stored calculated snapshot can supply a live value. Saved scalar and dictionary identity alone do not establish either relation.

### DI-04. Configured-property resolver and legacy ownership

**Question.** What complete model/configuration/property resolver and legacy carrier grammar admit a configured property operand without borrowing another namespace or stale calculated value?

**Known.** `sldprt.md` §8.1 "Modern configuration custom properties use" defines the exact modern configuration-name/stored-ID/carrier join and its UserDefinedProperties section. Stored IDs differ from element positions. Default, document custom and configuration custom properties are separate namespaces.

**Need.** Independent legacy configuration-property dictionary/FMTID ownership, no-ID admission, complete operand binding, and strict retained-writer behavior for configuration-local property edits. A saved formula does not establish currentness or override carrier ownership.

**Conflict.** The current configuration projection retains Keywords fields but does not incorporate the modern configuration-property namespace. The saved default-property reader has a separate document owner and does not resolve configured custom properties.

**Note.** Modern name/ID evidence does not establish a legacy singleton fallback, property-cache freshness, or complete evaluator execution.

### DI-05. Compact offset framing and complete operand ownership

**Question.** What complete compact/current marker framing and source/result/driver/orientation graph admits an offset-edge relation?

**Known.** `sldprt.md` §2 "An offset-edge marker relation retains native handles" distinguishes raw marker operands from a complete offset constraint. Line supports can be parallel at one positive separation while their bounded lengths differ. Repeated handles, state and sentinel values do not supply a positive offset magnitude or a complete owner join.

**Need.** An independently established compact discriminator and record boundary, shape-specific local-ID handling, all source/result curve pairs, one shared positive driving magnitude, source direction and chain membership. General-curve offsets and nonlocal source projection require independent ownership and regularization evidence.

**Conflict.** The current link parser uses the current marker layout and misses the compact pair. Its local-ID caller assumes the current record extent when links parse successfully; adding a shorter link form without changing that caller can read the following record. A whole copied compact header does not establish the role or allowed domain of each header word.

**Note.** Retain raw markers and do not infer a compact local ID, a generic fixed header word, or a neutral offset from geometric coincidence.

### DI-06. Arc-cardinal relation invariants

**Question.** What neutral geometric invariant does each top, bottom, left, and right arc-cardinal marker relation impose?

**Known.** `sldprt.md` §2 "A `Config-N-ResolvedFeatures` lane supplies the evaluated parameter state for configuration slot" through `sldprt.md` §2 "A primary line-or-circle geometry handle on a transformed line segment identifies that line" define operand resolution and the neutral invariants of the supported relation families.

**Need.** We must know the invariants to project these native relations as neutral sketch constraints.

### DI-07. Ambiguous relation-locus ownership

**Question.** Which profile locus does a relation operand select when its marker graph identifies a handle but does not identify one geometric locus?

**Known.** `sldprt.md` §2 "A Keywords configuration's decimal `id` attribute is the slot identity for" through `sldprt.md` §2 "Feature-input geometry-handle coordinates and the nested Parasolid profile differ by a signed" define handle chains, coordinate transforms, canonical shared-coordinate loci, and relation operand selection. Ambiguous chains and transform-dependent markers remain unresolved.

**Need.** We must select one locus to construct the neutral constraint.

### DI-08. Relation codes `29..32`, `36..41`, and `43..85`

**Question.** What neutral invariant and operand roles does each relation code in `29..32`, `36..41`, and `43..85` have?

**Known.** The native numeric taxonomy defines identities through code `85`. `sldprt.md` §2 "A `Config-N-ResolvedFeatures` lane supplies the evaluated parameter state for configuration slot" through `sldprt.md` §2 "A primary line-or-circle geometry handle on a transformed line segment identifies that line" define the neutral semantics of the supported codes.

**Need.** We must know each invariant and operand family to project the remaining codes.

### DI-09. Relation codes above `85`

**Question.** What native relation does each code above `85` identify?

**Known.** The defined native numeric taxonomy ends at code `85`.

**Need.** We must know the identities before we can define their operands and neutral invariants.

### DI-10. Ambiguous marker transforms

**Question.** Which marker-to-profile transform applies when coordinate sets permit more than one signed-axis transform and the transforms select different loci?

**Known.** `sldprt.md` §2 "Keywords length literals use the suffixes `uin`, `mil`, `mm`, `cm`, `in`, `ft`, `nm`, `um`, `µm`" through `sldprt.md` §2 "Operand tags" define transform selection, placement fallback, and the case in which all valid transforms give the same locus set.

**Need.** We must select one transform to bind each remaining marker to profile geometry.

### DI-11. Ambiguous linked reference markers

**Question.** Which profile entity does a reference marker select when its linked loci do not identify one entity?

**Known.** `sldprt.md` §2 "A Keywords configuration's decimal `id` attribute is the slot identity for" and `sldprt.md` §2 "Keywords length literals use the suffixes `uin`, `mil`, `mm`, `cm`, `in`, `ft`, `nm`, `um`, `µm`" define unique linked-handle and shared-entity resolution.

**Need.** We must select one entity to bind the reference operand.

### DI-12. Auxiliary radial framing and diameter ownership

**Question.** What complete auxiliary-record discriminator and source relation bind a radial reference, its center, diameter and construction state?

**Known.** The exact compact legacy and extended radial layouts, their native code and role fields, the `sgSlot` relation, and the current-prefix geometry-locus `sgArcHandle` point carrier identify supported dimensioned-circle records. A declared `sgEntHandle` operand that references the exact point carrier supplies its center even when no radial record is present. Tags `d4 80` and `d5 80` directly carry a finite same-feature point or constrained-point center whose local identifier equals the operand address; their explicit identity takes precedence over pair and radius searches. Other native `sgCircleDim` tags use that same explicit point identity only after no declared pair, child, geometry-locus, or unique circular-witness carrier resolves. The reference must resolve in the unique declaring `sgEntHandle` lane to a finite same-feature point or constrained-point marker whose local identifier equals the operand address. A point that is the radial member of a declared pair cannot supply this fallback center; an invalid or ambiguous identity remains native. A declared `sgEntHandle` operand also accepts an adjacent center/radial point pair when the center local identifier equals the radial object index; the radial local identifier is either absent or zero in the ordinary form, or nonzero in the indexed point form. Tag `6e 83` uses an unreferenced lane-local `sgEntHandle` to select an indexed adjacent point pair from an even coordinate-bearing roster; every pair must carry the center-local-to-radial-object join and a nonzero radial local identifier. A declared `sgEntHandle` operand with an explicit `sgSlot_c` marker accepts one unique scoped `sgSlotHandle` class with exactly two current `e7 88` cells: the first cell identifies the slot and the second identifies one of the slot's selected center points. That form carries a construction circle centered at the selected point. A coordinate-less `line_or_circle` marker before the operand with exactly two identical links to one earlier same-feature `Arc` marker is also an arc carrier when both local identifiers match and the arc passes its endpoint or unique-later-radial-witness validation. Multiple, mismatched, or invalid linked carriers remain native. The selected profile stream can omit construction circles.

Two omitted owner-qualified construction circles have separate current/extended auxiliary envelopes with repeated words that match feature-local point object indices. One matched object index is `7` while its coordinate-roster ordinal is `5`. These address spaces cannot be interchanged. The API circle and stored diameter agree numerically, but that agreement does not establish the auxiliary-to-diameter/center ownership graph.

**Need.** Independently established framing with opaque header words left opaque, the repeated field's address domain, and an explicit owned diameter/center association or a separately qualified owner rule. Preserve unique raw records and refuse ambiguous center, radial or diameter joins.

**Conflict.** A whole copied auxiliary header fixes an unassigned word as a discriminator. A specimen-supplied diameter followed by a unique radius match does not prove that the record names that diameter or center. The current Native marker collector and circle projection omit the two auxiliary records and owner circles; this remains an implementation gap.

**Note.** A construction observation does not establish a generic byte discriminator. Missing supported radial records do not imply construction. The reported historical construction/absence census remains outside the bounded evidence.

### DI-13. Marker-only profile placement

**Question.** How is a marker-only profile placed in model space when it has no local, contextual, or unique lane-wide compact reference-plane record?

**Known.** `sldprt.md` §2 "Point-reference cells address feature-local object indices within the owning feature object" defines model-space recovery from a planar sketch placement. `sldprt.md` §2 "An `moLPattern_c` feature-input object is immediately preceded by its seed feature object. That" through `sldprt.md` §2 "Among classless, parameterless, propertyless history records, `Feature` source ID `1` is the" define the supported reference-plane frames.

**Need.** We must know the placement to transfer the profile from planar coordinates to model coordinates.

### DI-14. Other Keywords operation families

**Question.** What neutral operation does each Keywords operation family outside the typed neutral feature set represent?

**Known.** `sldprt.md` §2 "A detailed curve record is immediately followed by a curve-detail marker of the same generation:" through `sldprt.md` §2 "A bare `0` or `1` Keywords dimension bound to a unique driving distance" define the native records and operands of the supported history operations. Projected split-line operations are defined by `sldprt.md` §2 "An `moPLine_c` feature-input object containing exactly one `moPLineProject_c` class" and are outside this open item.

**Need.** We must know the operation semantics to transfer the remaining design history.

### DI-15. Up-to-vertex endpoint selector

**Question.** Which endpoint does the u32 selector in an up-to-vertex code-`3` edge-endpoint reference select?

**Known.** The point-reference form retains the edge and endpoint selector. The decoder retains the little-endian u32 at four bytes before the duplicated selection-vector marker as `endpoint_selector`. `sldprt.md` §2 "A `Config-N-ResolvedFeatures` lane supplies the evaluated parameter state for configuration slot" states how ordinary point-reference object indices resolve.

**Need.** We must know the selector values to terminate the extrusion at the correct edge endpoint.

### DI-16. Point-reference codes above `9`

**Question.** What point-reference form does each code above `9` identify?

**Known.** Point-reference codes through `9` have defined native forms and neutral projections.

**Need.** We must know the forms to project their selected geometry.

### DI-17. Other second-direction termination codes

**Question.** What termination does each second-direction end-spec code other than `0` and `1` represent?

**Known.** `sldprt.md` §2 "A named feature-input object bound to a classless history `Sketch` record with a nonzero source" defines second-direction code `0` as absent and code `1` as through-all.

**Need.** We must know the other codes to construct the second extrusion extent.

### DI-18. Other first- and second-direction combinations

**Question.** What semantics apply when a nonzero second-direction code occurs with a first-direction code other than `0`, `1`, or `9`?

**Known.** `sldprt.md` §2 "A named feature-input object bound to a classless history `Sketch` record with a nonzero source" defines the supported one-direction and two-direction combinations.

**Need.** We must know the combination semantics to construct both extrusion extents.

### DI-19. Reference-bearing end-spec shapes

**Question.** What record uses the end-spec shape when the word at `+18` is a reference child instead of a termination code?

**Known.** `sldprt.md` §2 "A named feature-input object bound to a classless history `Sketch` record with a nonzero source" defines the end-spec anchor and the termination words at `+18` and `+22` for valid end-spec children.

**Need.** We must distinguish the other record family from an extrusion end specification.

### DI-20. Feature-local face reconciliation

**Question.** What is the disposition of a feature-local face identity when no surviving face carries that identity?

**Known.** `sldprt.md` §2 "An extrusion object-name record is followed by four zero bytes, a little-endian u16 family word," through `sldprt.md` §2 "When operation objects and their dimension children form separate ordered groups, a blind" define the `moSingleFaceRef_w` path forms and their feature-local face selection. `sldprt.md` §4.1 "The fourth FACE reference is the owning SHELL." defines the typed face relation, and §5 defines how the terminal owner and feature-local face identity select the surviving face through `FACE_ID_2001`.

**Need.** We must distinguish a face consumed by a later operation from a face whose owner or identity attribute was not decoded.

### DI-21. Class-scoped inline extrusion operations

**Question.** What Boolean operation does the inline family `0x0140` with operation byte `00` represent for `moCutThin_c`, and what operation does each other unsupported family-word and operation-byte combination represent?

**Known.** `sldprt.md` §2 "Feature-tree" defines the supported ordinary extrusion combinations. The ordinary `0x0140`/`00` join mapping does not establish the operation of a `moCutThin_c` object. Family `0x01ca` with byte `00` or `02` establishes cut. A normalized Keywords `CutExtrudeThin` type also establishes cut independently. A complete configuration-local operation carrier is distinct from the base configuration's operation state.

**Need.** We must identify an independent, class-scoped Boolean discriminator for the `moCutThin_c` `0x0140`/`00` form and define the remaining family-word and operation-byte combinations.

### DI-22. Other sparse extrusion form codes

**Question.** What operation and record shape does each sparse extrusion object outside the defined form codes use?

**Known.** `sldprt.md` §2 "Keywords element order is serialization order, not regeneration order. Neutral regeneration" through `sldprt.md` §2 "Feature-tree" define direct, repeated-class, sentinel, terminated-trailer, and sparse-trailer objects. Form code `11` identifies an object form and does not determine its Boolean operation. A Keywords operation token or a complete inline operation trailer supplies the operation. Without either carrier, the operation remains unresolved and the decoder reports a design loss.

**Need.** We must know the remaining form codes to parse each object and construct its Boolean operation.

### DI-23. Combine-body reconciliation

**Question.** How does a body selected by a compact `moCombineBodies_c` target or tool path map to a body in the final B-rep?

**Known.** The compact paths identify generated feature-local bodies. A complete target or tool path projects to one generated body reference whose feature is the terminal path producer, whose local identity is the ordered path-local sequence, and whose dependencies include every uniquely identified traversed history feature. `sldprt.md` §4.3 "00 01 00 01" through `sldprt.md` §6 "Body membership is the typed XT ownership graph." define final B-rep body identities and ownership. `FACE_ID_2001` carries face identities and does not bind a feature-local body identity.

**Need.** We must know the mapping to bind the Boolean target and tool bodies.

### DI-24. Delete-body reconciliation

**Question.** How does an `moDeleteBody_c` regeneration-input-local body identity map to a body in the final B-rep?

**Known.** The compact record retains the regeneration-input-local identity. `sldprt.md` §4.3 "00 01 00 01" through `sldprt.md` §6 "Body membership is the typed XT ownership graph." define final B-rep body identities and ownership.

**Need.** We must know the mapping to delete the selected final body.

### DI-25. Component-edge path reconciliation

**Question.** How does an edge selected by an entry-form `moCompEdge_c` path map to an edge in the final B-rep?

**Known.** The entry path identifies a generated feature-local edge. `sldprt.md` §3.2 "An attribute" through `sldprt.md` §4 "**Bridge `00 0e`:** `attr` is u16 BE at body +0" define final B-rep edge identity and direction. `FACE_ID_2001` carries no edge identity. The `EDGE_PERM_ID_2_2003` attribute grammar is not defined.

**Need.** We must know the mapping to bind the selected operation edge.

### DI-26. Compact edge-vector B-rep reconciliation

**Question.** How does a compact edge vector map its feature-local edge path to a final B-rep edge?

**Known.** `sldprt.md` §2 defines the duplicated-marker framing, count, lane-local selector, typed and reference-list entry forms, separators, terminal slots, and lane-local type signatures. The decoder retains the ordered compact vector and its producer-feature paths. The terminal component identifies a feature-local generated edge, but it does not by itself identify the final B-rep edge.

**Need.** We must define the topology join from the retained compact edge identity to the final B-rep edge before projecting the selected edge.

### DI-28. Sweep reference child versions

**Question.** Which complete class and version grammar selects the shorter component-profile source record, and which source-less forms establish an operand owner?

**Known.** A general-curve reference identifies the sweep cross-section. A component-reference curve identifies its path. The supported complete source records identify a feature independently of neighboring feature order. An unresolved reference does not establish a sketch or a final B-rep curve.

**Need.** We must identify each version before reading its source field and prove the reference namespace before selecting geometry.

**Conflict.** A neighboring sketch does not replace an explicit reference whose child form is unsupported.

### DI-29. Composite sweep path identity

**Question.** How do the complete ordered component instances and persistent identifiers select the curves in an earlier feature result?

**Known.** The component-reference curve is the path operand. Its type signatures, instances, identifiers, and order are separate parts of the reference. Repeated local identifiers do not establish repeated curve instances. A resolved producer does not establish a final B-rep edge.

**Need.** We must bind the complete producer-result identity before representing individual generated path curves. Preserve the reference record when that join is unavailable.

### DI-30. Other compact sweep Boolean codes

**Question.** Which serialized carrier establishes the solid-sweep Merge Boolean independently of the compact class-form word?

**Known.** The Merge property is Boolean. An unrecognized compact word does not establish NewBody, Cut, or Intersect. A missing carrier does not establish Merge false. An owned unknown, malformed, or ambiguous operation cannot corroborate another lane.

**Need.** Bind the complete Merge carrier to its owning feature with independent true and false controls. Retain unresolved operations when the carrier is unavailable.

### DI-31. Native neutral-plane Draft outward flag

**Question.** Which native field identifies the outward flag of a neutral-plane Draft when Keywords omits it?

**Known.** `sldprt.md` §2 "A Draft feature-input interval uses the lane-scoped `moPlaneRef_w` token" defines neutral-plane Draft operands. `sldprt.md` §2 "A compact parting-line Draft uses direct duplicated component-vector records" defines the parting-line form, which has no outward operand. Keywords independently retains an explicit `Outward` Boolean for a neutral-plane Draft.

**Need.** We must recover the native outward flag of a neutral-plane Draft when Keywords omits it. The signed declared pull vector does not establish an unflipped neutral-plane orientation or the outward Boolean. Compact parting-line frame direction semantics require separate Source controls; they do not share the declared +80 field.

### DI-32. Other compact line-reference widths and owned pattern units

**Question.** Which complete enclosing grammars select additional compact line-reference directions, and which owned metric fields establish the corresponding pattern spacings?

**Known.** The complete 136-byte token-terminated eight-scalar form stores its unit vector at +64. Its trailer and next-object boundary select the width. An overlapping unit triple does not select another width. The raw display-spacing fields and bare history literals are separate carriers; a new direction does not establish the literals' length unit.

**Need.** Bind the complete dimension-record framing, feature owner, role and unit before enabling a pattern extent. Preserve unknown directions or spacing semantics when those joins are unavailable. Do not apply a document-wide scale from numeric agreement.

### DI-34. SWIFT implicit nominal construction

**Question.** Which nominal-geometry rule applies to each remaining feature-size annotation whose `Nominal` field is zero and whose `Dimension` field is absent or zero?

**Known.** `sldprt.md` §2.1 defines zero as an omitted nominal sentinel when `Dimension` is absent or zero. It defines diameter and depth nominal recovery from the rendered literal, declared decimal places, pattern or compound-feature traversal, and cylindrical or spherical nominal geometry. The rendered literal, not the unrounded geometry, supplies the labeled value. It defines width and length from named slot-geometry fields and radius from the named radius field of fillet, cylindrical, or spherical geometry. It defines counterbore diameter from the direct nominal cylinder and countersink diameter and angle from the direct nominal cone. An empty applied-feature graph and empty CAD identifiers supply no direct geometry and do not bind an unrelated rendered literal; an empty `GdtPattern` can additionally use the exact `Hole PatternN` to `LPatternN` history join when one seed has exactly one later consuming Hole with a positive diameter. Directional distance annotations recover plane, axis, compound-hole, and closed-slot feature-of-size locations from their named geometry.

**Need.** The empty-pattern history join is settled by `sldprt.md` §2.1. We must still define nonidentity `NominalTransform` semantics and the other `ComputeAnswerBy`, `Direction`, and `NormalTo` modes before those forms can supply a nominal. A zero direct-cylinder `ThreadDepth` does not establish an evaluated thread depth. Recovering that form requires a source-owned association between the annotation and its rendered depth, and a rule that distinguishes an omitted thread depth from an authored zero. A file-wide matching hole-depth literal and ordinary cylindrical extent do not establish that association.

### DI-35. Variable-fillet endpoint grouping and control ownership

**Question.** Which complete repeated dimension-object grammar separates the selected-edge roster from radius controls, and how do intermediate controls bind to the full radius law?

**Known.** Within one owned variable-fillet object, two proper reference subsets associate the final two selected references with their retained-endpoint radii. The full ordered component identity distinguishes references. A control equal to the whole selected vector does not identify an endpoint. Endpoint associations and complete radius-law controls are separate relations.

**Need.** Prove the complete counted and typed repeated-object boundary in every selected/control caller. A marker sequence alone does not establish that boundary. Bind each scalar through its owned named dimension record, preserve intermediate and exact-equal controls, and establish their parameter positions before constructing the full law. Preserve the existing explicit-endpoint and legacy contracts without inventing endpoint knots or a final B-rep mapping.

### DI-36. SurfaceCut discarded side

**Question.** Which native field in an `moSurfCut_c` interval selects the retained side of a surface cut?

**Known.** `sldprt.md` §2 defines the target-body reference-list vector and the cutting-surface component vector. The target projects as a body selection and the cutting surface projects as a face selection. The neutral `CutWithSurface.reverse` value remains optional; an absent native Boolean remains unresolved.

**Need.** Distinguish the wrapper selector byte and the tail words between the surface-body identity pair and the termination sentinels with labeled parts cut to opposite sides before assigning the reverse value.

### DI-39. Temporary-axis ownership in revolution features

**Question.** Which owned revolution input identifies the temporary-axis carrier when a profile-source join or another line-reference candidate is also present?

**Known.** `sldprt.md` §2 defines the nine-scalar carrier's middle point, direction, exact compact or declared boundary, and valid zero stream address. Those carrier fields do not establish which feature owns the axis or whether another independent source selects a different axis.

**Need.** A complete profile and axis ownership relation must connect the carrier to the revolution. Competing complete source operands must agree before the axis is admitted; source order and fallback priority do not establish agreement.

### DI-40. Extrusion profile identity without an explicit profile relation

**Question.** Which native relation binds the profile of a classless
`Extrusion` or `DissectableRoot` extrusion when `Profile` and
`DissectableChildren` do not identify a sketch?

**Known.** An explicit `Profile` attribute or a single dissected sketch child
binds the profile. Source identifiers order feature records, but ordering alone
does not establish that an earlier sketch is the extrusion's regeneration
input.

**Need.** We must decode the profile dependency before projecting the extrusion
with a neutral sketch profile.

**Note.** A unique unused earlier sketch does not establish a universal no-reuse rule. Profile ownership and configuration-scoped absence or conflict require independent source relations; an unsupported provisional choice must not block an earned relation.

### DI-41. Pattern seed identity without a resolved seed relation

**Question.** Which native relation binds the seed of a linear or mirror pattern
when its component vectors and persistent generated-surface identities do not
resolve a seed feature?

**Known.** Complete component paths and generated-surface identities retain provenance, but do not certify a complete selected-feature roster. Independent authored selections include a two-seed mirror whose paths mention only one seed. A derived cosmetic-thread output identifies a generated child, not its input seed. Feature order does not establish ownership. The reader withholds nearest-thread, preceding-object and mirror-provenance seed guesses while preserving explicit seed relations, dimensions, directions, mirror planes and Native records.

**Need.** Decode the complete manager/instance-holder ownership and selected-list grammar, including configuration-scoped absence and conflicts. A bounded search witness or unique provenance candidate cannot establish complete selection cardinality. Circular and curve-driven seed families require their own independent selection proofs.

## 6. Container record semantics

### EV-03. `SWObjects` record content

**Question.** What fields and relations occupy the unresolved bytes of each `SWObjects` metadata record?

**Known.** The supported section-root Part scalar forms are four or twelve bytes with an exact class-specific continuation; a class string in opaque data does not establish a Part. Configuration-manager prefixes are versioned and own child graphs, so the former fixed offsets do not establish scalar meanings or a record end. The reader retains these manager bytes without editable fields, and both writer paths refuse the unsupported scalar attribute. Appearance tails remain versioned; public color APIs alone do not prove the serialized nonprimary color or material-field assignments.

**Need.** We must know what the undecoded bytes hold before the writer can edit a retained record or claim that a generated record is complete.
