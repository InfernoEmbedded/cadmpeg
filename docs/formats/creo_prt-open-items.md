# Creo Parametric `.prt` (PSB): Open Items

This document lists the parts of the Creo Parametric `.prt` format that we do not know. The specification `creo_prt.md` gives the parts that we know.

Each item has these parts:

- **Question** — what we must find.
- **Known** — what the specification gives now.
- **Need** — why we must find the answer.
- **Conflict** — a disagreement between two documents, or between a document and the decoder. An item with this part needs a decision.
- **Note** — a defect in the item or in the specification.

When an item is resolved, delete it in the same change that writes the answer into the specification. Do not keep a Resolved part.

Each item has an identifier. Use the identifier in commit messages and in code comments.

This document uses ASD-STE100 Simplified Technical English. Record names, field names, and token values are technical names. They keep their source spelling.

## 1. Scalar and array encoding

### SE-01. Custom relation units

**Question.** How does each custom unit symbol convert a curve-relation value to canonical units?

**Known.** `creo_prt.md` §8.3 "Square brackets following a numeric literal or parameter expression contain a" defines the built-in unit symbols, their dimensions, and their canonical conversions. Celsius and Fahrenheit use affine conversions.

**Need.** We must know each custom conversion to evaluate a relation that uses the symbol.

### SE-02. Owned custom affine-unit definitions

**Question.** Which complete basic custom-unit record binds its quantity type,
reference unit, scale, and offset to one model-local unit name?

**Known.** `creo_prt.md` §8.3 defines the reference-to-actual conversion and
its inverse. A nonzero offset belongs only to basic temperature. The separate
`xp_custom_sys_unit._conv_factors` array holds system-conversion pairs and
cannot assign an offset to a named custom unit.

**Need.** We must know the complete nonempty definition grammar and reference
ownership before resolving model-local affine units, including missing,
ambiguous, or cyclic references.

### SE-03. Other curve-equation frame transitions

**Question.** Which additional stateful productions are owned by a
curve-equation `local_sys` body beyond the defined explicit slot language?

**Known.** `creo_prt.md` §8.3 defines the curve-owned wrapper and twelve frame
slots. The complete rank-two image `18 e4 0f e4 18 e5 0f 18 e6` consumes nine
bytes and supplies all twelve slots. Its middle triple is zero; splitting its
`e4` bytes into generic unit scalars changes that rank. The next named field
bounds the owned body. Active and backup images of one saved equation are
persistence copies, not distinct transition productions.

**Need.** We must know the complete curve-owned byte extent, slot transition,
and independent placement constraints before admitting another stateful
production. A common `f9 04 03` wrapper does not establish that feature,
surface, or pcurve transition semantics apply.

### SE-04. Other pcurve array transitions

**Question.** What slot transition does each other stateful `crv_pnt_arr f9 02 04` token encode?

**Known.** `creo_prt.md` §4.1 "A direct curve body consisting of exactly eight scalar slots and no references" defines the direct eight-slot pcurve body, its endpoint order, standalone zero slots, and the exact held-scalar wrapper.

**Need.** We must know the other transitions to decode all pcurve endpoint arrays.

### SE-07. Other `double_xar` slots

**Question.** What grammar does each `double_xar` slot use when it is not a defined literal, scalar, placeholder, or terminal slot?

**Known.** `creo_prt.md` §2.3 "An expanded" defines the counted array, literal-one slot, literal-zero slot, two recursive placeholder images, scalar slots, and terminal null slot.

**Need.** We must know the other grammars to keep the counted slot boundary aligned.

### SE-08. Variable-length `e5` slots

**Question.** What bounds and semantics apply to a variable-length `e5` slot in `double_xar`?

**Known.** `creo_prt.md` §2.3 "An expanded" defines `e5 07 23 11 2e` as one exact recursive placeholder image.

**Need.** We must know the bounds to distinguish one slot from following slots.

### SE-11–SE-13. Short unresolved solver-body tails

**Question.** What solver identities or state do the tails of the bounded `00 <tail2>`, `01 <tail3>`, and `34 <tail2>` variable bodies retain?

**Known.** `creo_prt.md` §5 defines their exact widths, independent value and guess lanes, and unresolved inline state. A complete enabled section graph can resolve a row value separately; equal short bodies do not establish equal numerical values across sections.

**Need.** Define the owned tail fields and their joins before assigning additional solver semantics or replacing the unresolved lane state.

### SE-14. Other principal unit selectors

**Question.** Which coordinate unit system does each `_principal_sys_units_id`
value outside `51`, `54`, and `55` select?

**Known.** `creo_prt.md` §1.3 "`_principal_sys_units_id` identifies the active
coordinate unit system." gives `51` for millimeter-Newton-Second, `54` for
inch-pound-mass-second, and `55` for millimeter-Kilogram-Second. Binary
selector `54` stores lengths in inches and the neutral model multiplies every
length-bearing value by `25.4` to produce canonical millimeters. `creo_prt.md`
§1.3 "In legacy ASCII persistence, the unique type-10
`principal_sys_units` scalar" gives the same inch system and conversion.

**Need.** We must know each selector to give model coordinates their canonical
length unit.

## 2. Curves and surfaces

### GS-01. Cone half-angle overrides

**Question.** What grammar and value rule apply to a per-instance cone half-angle that is not the terminal positive-DICT form?

**Known.** `creo_prt.md` §3.2 "A positional cone suffix consists of exactly one complete nine-slot support" defines the terminal positive-DICT half-angle form and the support-frame construction.

**Need.** We must know the override to construct the cone carrier.

### GS-02. Torus and sphere radius overrides

**Question.** What complete grammar and placement rule apply to a `geom_type = 26` body outside the tagged radius trailer, complete inline local-system plus radius pair, and terminal prototype-minor-radius replay forms?

**Known.** `creo_prt.md` §3.3 "A `srf_prim_ptr(torus)` prototype stores" through `creo_prt.md` §3.3 "In named `radius`, `radius1`, and `radius2` fields" define the recognized torus and sphere radius forms and their carrier invariants.

**Need.** Define the remaining body grammar and a unique placement witness before constructing a torus or sphere carrier from those forms. An agreeing inline radius pair does not resolve conflicting origin or direction readings.

### GS-06. Prototype-adjacent `tab_cyl` parameters

**Question.** What does each parameter in a prototype-adjacent `tab_cyl` instance row control?

**Known.** `creo_prt.md` §3.2 defines the complete type-19, degree-three, four-control-point knot production and the separate interpolation-point parameter production. The knot production fixes the directrix domain `[0, 1]`. It does not establish the roles of other instance parameters.

**Need.** Establish the remaining instance parameter roles and their ownership before deriving a surface chart or placement.

### GS-07. Ambiguous `tab_cyl` placement

**Question.** How does a replay-bound `tab_cyl` select its placement when its axis span matches neither directrix-coordinate range uniquely?

**Known.** `creo_prt.md` §3.2 "A repeated `tab_cyl` cubic-curve replay has this structure:" defines the repeated cubic replay and the unique axis-span placement cases.

**Need.** We must know the selection rule to place the surface in model space.

### GS-08. Unresolved `fc 02` bodies

**Question.** Which scalar images or additional body productions resolve an `fc 02` body that does not satisfy the complete counted or complete short production?

**Known.** The complete counted production has eight endpoint slots in two face charts. The complete short production supplies one face path. The bounded microphone and heat-sink verification resolves 72 counted bodies and two short bodies; 68 other bodies remain opaque. An opaque body can contain an undefined scalar in a known production. Opacity alone does not prove another grammar.

**Need.** Resolve each remaining body through complete scalar ownership and chart roles before it supplies curve or endpoint geometry.

### GS-09. Other `fc 05` variants

**Question.** What grammar does an `fc 05` body use when it does not satisfy the defined cap-circle production?

**Known.** `creo_prt.md` §4.2 "`fc 05` records store cap-circle control points in the order `A`, `B`, `t`, `C`, where `A` and" through `creo_prt.md` §4.2 "One `fc 05`" define complete point groups, scalar lanes, termination, cylinder binding, placement, and circle construction for cap-circle bodies.

**Need.** We must know the other grammar to construct or reject the curve correctly.

### GS-10. `fc 08` grammar

**Question.** What is the complete body grammar for `fc 08`?

**Known.** `creo_prt.md` §4.2 "An `fc` prefix is resolved by exact body grammar." identifies `fc 08` as a world-coordinate control-polyline family. Recognized coordinate tokens and opaque spans partition its retained body.

**Need.** We must know the grammar to construct the control polyline.

### GS-11. `fc 13` scalar signs and model-space charts

**Question.** How do the remaining undefined scalar images, signed coordinate branches, and native chart transforms define complete placed paths?

**Known.** The complete point-first and parameter-first productions associate one parameter with three coordinate roles. Their shortened terminal group completes the preceding parameter's sample. These roles do not prove a spatial control polyline, NURBS control poles, or a native-to-model transform. A radius identity alone does not select a signed branch. The panel 687 path retains unresolved branch signs; the CTC02 and microphone role proofs do not establish their complete native model-space placements. No named nineteen-by-four array declaration establishes a universal interpretation of `13` as a sample count.

**Need.** Resolve the undefined images and derive each complete signed chart from native ownership before constructing placed geometry.

### GS-13. Incomplete counted pcurve bodies

**Question.** Which remaining scalar and enclosing-row forms resolve a counted two-chart body when exact `4N` scalar consumption cannot be established?

**Known.** `creo_prt.md` defines `fc <count>` with four finite chart scalars per sample and no trailer. Counts `4`, `7`, `9`, and `10` do not identify separate curve families. A unique complete counted body fixes a competing topology-suffix boundary without requiring the referenced face to be materialized.

**Need.** Define the missing scalar and enclosing-row productions before assigning values or selecting a suffix from incomplete or conflicting body readings.

### GS-16. Model-reference entity types and conic framing

**Question.** Which complete entity grammars own the unresolved positional
records after a conic declaration, and how do their types select carriers?

**Known.** Type 30 identifies a model-reference ellipse. A preceding conic-list
declaration does not identify a later row's type or grammar. The solid-entity
and sketch-entity type namespaces are distinct. An integer type alone does not
establish a parabola or hyperbola. Ellipse construction requires the complete
frame, coefficients, and endpoint invariants in `creo_prt.md` §8.5.

**Need.** Define each complete owned record production before assigning its
fields or carrier. Resolve positional schema references before the flip field
and the complete twelve-slot local system. Agreement of the center and radii
alone does not establish the frame or endpoints. Preserve unresolved rows
without inheriting a grammar from a preceding list declaration.

### GS-17. Other positional cylinders

**Question.** What model-space equation does each positional cylinder body outside the defined analytic and generated round-edge forms encode?

**Known.** `creo_prt.md` §3.2 "`srf_prim_ptr` records contain the surface prototype fields" through `creo_prt.md` §3.2 "Positional cylinder rows store cap-plane point data rather than a `local_sys` replay." define the recognized cylinder row families and their placement invariants, including the `11 10 13` placement-witnessed inline cylinder and generated type-24 round-edge endpoint bodies. The first directrix-coordinate positive DICT lattice covers prefixes `4b..a3`.

In the seven-scalar directrix-coordinate lane, the first scalar is positive. It
usually equals the axial corner span: example pairs include `1.7` and `1.7`,
and `0.74` and `0.7399999999999998`. It does not always equal that span: one
example has a first scalar of `1.78` and an axial corner span of `1.68`. Its
role is not known.

A fully framed cap-envelope body owned by a unique cylinder row and resolved
class 932 remains opaque. A bounded station/envelope and a radial suffix can
agree with stored circles without defining the chart direction, phase, or
initial control roles. The generic terminal-support frame does not establish
those roles. Incomplete centers remain unresolved; an absent center cannot be
replaced by an envelope midpoint. Raw body bytes retain the unsupported fields.

The selector-corner interval family reconstructs its unique axis line, radius,
and extent from two directrix parameters, two XYZ corners, and the selector
pair. Its split-selector and one-placeholder variants use the same equation.
The four-bound inline family selects either the positional row outline lane or
the signed first-directrix outline lane from its first outline token. Both
forms reconstruct a complete carrier from the inline local-system suffix.
Class-913 axial-interval corner bodies without a quadrant selector retain four
axis-line candidates. A unique maximum count of adjacent tangent support planes
selects the carrier; a tied or zero maximum does not.
Their common transverse span is a rolling-radius sample. In a
round-edge endpoint body, the unique pair of equal nonzero endpoint-coordinate
deltas supplies the perpendicular-round radius. In a class-913 generated-round
context, axial-interval and endpoint bodies do not fall through to scalar-frame
or terminal-suffix radius grammars. Outside that context, a byte-compatible
body does not establish endpoint ownership.

Class-913 generated round-edge replays carry a radius candidate in a bounded
`cr_flags_xar` positional record. Its `f2 f7 80 a0` opener, following
`01 f6` compact-field terminator, first `0x29` short-form scalar, and next
`f3 f7 80 97 e2` boundary are defined in `creo_prt.md` §3.2. The candidate
is used only with agreeing observed rolling-radius and placed-cylinder
witnesses; it does not by itself define the remaining carrier equation.

**Need.** We must know the equation to construct the cylinder carrier.

### GS-19. Unresolved cone origin-coordinate images

**Question.** Which owned scalar or model-reference grammar resolves the remaining cone origin-coordinate images and establishes their placement?

**Known.** `creo_prt.md` §3.2 defines the twelve-slot owner and the zero/reference/short origin production. Its last image is `origin[2]`, not a station parameter. The bounded `2f3400` value is source-length 20; the CTC05 inch selector converts it to 508 mm. Independent resistor STEP apices constrain the checked `1b`/`34` instances to zero Z but supply no raw-only formula. The CTC05 raw origin is `18`, `19c60fafbfd97304`, `2f3400`; interpreting the first two as independent generic numeric lanes gives X/Y values that disagree with the sibling STEP. Complete seven-byte scalar owners also prohibit splitting their suffix bytes into another coordinate.

**Need.** Resolve the reference-backed origin through its complete native owner before admitting a placed cone. Preserve unresolved bytes and withhold dependent geometry. Do not map a prefix to zero from the checked resistor instances or use a STEP fit as the runtime decoder.

### GS-21. Non-prismatic round radii

**Question.** Which class-913 instance fields encode round sets, their shape options, ordered governing-point radius entries, and radius-leg values or references?

**Known.** `creo_prt.md` §6 defines the per-set variable-round element contract and the resolved constant-radius forms. The named `data(subft_rndset)` definition retains chain, spine, point-set, and subset fields. Its point-set fields do not establish native instance station ownership. Unequal generated carrier radii define neither set boundaries nor a variable-radius law.

**Need.** A native instance with independently known element-tree values must establish set-array boundaries, the shape-option selector, radius-entry order, governing-point references, and the two leg grammars.

### GS-22. Round flank geometry

**Question.** Which fields define the flank geometry of a round or fillet?

**Known.** The feature recipe retains affected geometry, affected edges, contours, and generated entities as separate identifier arrays.

**Need.** We must know the fields to construct the blend surface and its trims.

### GS-23. Round carrier ownership and generated-entry extents

**Question.** Which complete native fields establish carrier/sense inheritance for a row-unresolved generated face, and which body grammar bounds the final plain generated-entity entry?

**Known.** `creo_prt.md` §6 defines complete generated-result identities through unique, same-owner surface rows across all owned tables. Current face-side identity and independent B-rep admission are separate joins. The bounded round example reproduces its complete nine-member result roster. This does not establish the general rowless-cylinder carrier claim or the final plain-entry grammar.

**Need.** Obtain complete source-owned rowless carrier/sense discriminators and plain-entry field consumption or a positively owned enclosing boundary. Preserve the observed overlap with a candidate following table header as an unresolved extent issue. A raw `f2 f7` search, a fixed byte subtraction, or table adjacency is not a boundary or carrier proof. Historical topology/census counts and inactive-item selection remain separate from this verified result-identity rule.

## 3. Section solving and feature placement

### SP-01. Nonlinear simultaneous equations

**Question.** How must a nonlinear curve-equation `SOLVE` block be solved?

**Known.** `creo_prt.md` §8.3 "`SOLVE` opens a simultaneous-equation block and `FOR` followed by one or more" defines the block framing. The decoder solves complete, dimensionally valid affine systems and smooth numeric nonlinear systems when one finite full-rank root is established, and retains other blocks.

**Need.** Obtain saved programs with regenerated numeric or geometry witnesses for non-smooth, piecewise, and discrete forms that do not reduce affinely. Vary preceding unknown values, equation order, and `FOR` variable order. The witnesses must distinguish the selected root or branch from nearest-seed and fixed-order rules.

**Conflict.** A simultaneous system can have more than one mathematical result set. The returned set does not by itself identify the initialization or branch rule.

**Note.** Preserve an unsolved block and invalidate its preceding unknown values until the complete source semantics supplies a unique transfer.

### SP-03. Section-to-datum joins

**Question.** Which additional fields join a section definition to its sketch datum when no unique bounded-source owner chain or generated-datum parent remains?

**Known.** `creo_prt.md` §6 "`DEPDB_DATA` and each complete bounded `AllFeatur` feature row store an" defines the recipe, consecutive datum, and `gsec3d_ptr.sketch_plane` join. The definition is eligible only inside its complete source range. The same section-plane entity used by multiple definitions remains ambiguous. `creo_prt.md` §6 "`dtm_id_tab [f1|f2] f8 <count> f7 <class> fb e2` is followed by exactly" through `creo_prt.md` §6 "When the sketch plane resolves to a placed plane carrier" define the unique generated-datum parent-table join and the `ActDatums` geometric identifiers.

**Need.** We must define a transfer for definitions that have no unique bounded-source chain and no unique generated-datum parent without inferring a feature owner or model-space frame.

### SP-04. Other relation equations

**Question.** What equation does each relation type outside signed type 0, type 1 angular, type 5, type 6, and type 14 encode?

**Known.** `creo_prt.md` §5 "Build the B-rep half-edge graph from the `crv_array` suffixes. A single-loop face has an outer" through `creo_prt.md` §5 "A positive-ratio elliptical cone uses local frame coordinates" define the recognized linear, radius, incidence, and entity-geometry relations. Complete `eqtn_arr` function-0, function-2, function-3, and function-35 rows define radial endpoint, scalar equality, unsigned coordinate distance, radius binding, and point-on-line equations when their positional row grammars are complete.
The supported type-six relation form has sign `1`,
`a=[first_point,second_point,0,1]`, `b=[center_point,0,0,0]`, and a complete
four-slot `c` selector; it binds a complete linear dimension to the unique arc
with the matching endpoint pair, center, and radius dimension index. Type four
produces a diameter constraint; the other linear dimension types produce a
radius constraint. An incomplete or ambiguous form remains native.
Function-13 rows with two type-2 point ordinates and a zero type-7 auxiliary
row define a same-coordinate equation.
Function-33 rows with four type-1/type-2 coordinate pairs identifying two
endpoint pairs and a zero type-7 auxiliary row define equality of the two
squared endpoint-pair lengths.
Function-6 rows with two complete type-1/type-2 point pairs and a type-3
scalar define their positive Euclidean distance.
Function-42 rows define the arithmetic-mean relation between two same-axis
point coordinates and a type-6 scalar. Function-31 rows bind one type-1/type-2
point pair to two type-6 coordinate scalars. Function-43 rows define the
eight-slot axis-distance form with two point pairs, two type-4-or-type-5
auxiliary rows, a type-0 distance row, and a type-5 auxiliary row. The
non-negative type-0 value transfers when it agrees with exactly one absolute
coordinate difference; a missing value transfers only when exactly one
coordinate difference is non-zero. Function-16 direct rows with two type-4
angle rows, a type-0 result row, and a zero type-5 selector transfer the
non-negative first-minus-second angle difference when it is at most π; a
missing result transfers from the two finite angles. Other function-16 forms
and other function-43 forms remain native. Function-10 seven-slot rows with
same-type selected coordinates for two distinct points and a target, opposite
type coordinates for the two reference points, and zero type-7 auxiliaries
transfer the target's missing opposite ordinate to the shared reference
ordinate when the selected ordinates differ and all other point identities and
coordinates are complete and unambiguous. Other function-10 forms remain
native.

**Need.** We must know each equation to solve the section geometry.

### SP-05. Type-1 direction selectors

**Question.** How does the complete `b`, `c`, and `sign` selector tuple choose the rays and their order for a type-1 angular relation?

**Known.** The relation retains its operand vectors, complete raw body, selector, and selected angular dimension separately. The line operands bind through the section order table. A scalar projection of `c` does not establish its direction semantics.

**Need.** Obtain regenerated section saves with fixed ordered line entities and endpoint order. Change only the dimension label sector, then reverse each line independently. Retain the ordered dimension references, point types, label location, and value from the section API. Compare the complete source tuple for each save.

**Conflict.** The angular magnitude and the stored `sign` do not establish the selected rays or a direction convention. A rule for labelled legacy sections does not establish the binary selector codebook.

**Note.** Retain the complete selector tuple. Transfer a directed target constraint only when its ray and direction semantics are independently resolved.

### SP-06. Other type-35 operands

**Question.** What does a type-35 operand identify when it does not resolve through a section entity?

**Known.** Section-entity identifiers and relation identifiers occupy separate namespaces.

**Need.** We must know the referent to evaluate the relation.

### SP-07. Non-coordinate full-width solver roles

**Question.** What numerical or solver role does a complete `ed <tail8>` image have in a variable outside coordinate types `1` and `2`?

**Known.** `creo_prt.md` §5 "Within each" defines the complete nine-byte full-width image. Coordinate types `1` and `2` decode its finite binary64 payload; other variable types retain an opaque scalar role. The image alone does not establish a dimension-driven state.

**Need.** Define the complete owning field and variable-type contracts before numerically admitting an opaque full-width variable or assigning solver state to it.

### SP-08. Multiple `local_sys` frames

**Question.** What geometric role does each feature-definition `local_sys` frame have when one definition contains more than one frame?

**Known.** `creo_prt.md` §6 "Feature-definition `local_sys f9 04 03` and `transf f9 04 03` bodies use the" defines the twelve-slot feature frame and its complete rank-two form.

**Need.** We must know each role to select the feature placement.

### SP-09. Multiple `transf` frames

**Question.** Which owner-specific role distinguishes complete `transf` frames if one bounded feature definition contains more than one?

**Known.** `creo_prt.md` §6 "Feature-definition `local_sys f9 04 03` and `transf f9 04 03` bodies use the" defines the twelve-slot body. A frame belongs to its bounded source definition and nested owner. A field with the same name outside that definition is not a competing frame.

**Need.** Obtain a definition that contains multiple complete transforms with independently identified owners and geometric results. Distinguish those roles before selecting a section-to-model transform.

**Note.** An absent, incomplete, or ambiguous owner-specific transform does not permit a file-wide lookup or selection by field order.

### SP-10. Selection within one frame owner

**Question.** If one parsed owning object contains multiple complete frames of
one kind, what instance identity, index, or selector distinguishes the field
required by its consumer?

**Known.** Complete `local_sys` and `transf` fields under different owners do
not form one selection array. The placement record, each transformation-array
entry, and the section saved-result retain separate owner/kind identities.
Order, equality, or completeness across owners cannot establish a section
location frame. A uniquely owned section frame is independent of unrelated
feature-placement and generated-carrier evidence.

**Need.** We must establish an actual within-owner multiple-frame grammar
before selecting one such instance. Missing, incomplete, or ambiguous evidence
in the required owner must remain unresolved; a complete sibling object is not
a fallback.

### SP-11. `relat_ptr` operand-vector roles

**Question.** Which remaining type-specific tuple shapes and selectors have transferable reference or scalar roles?

**Known.** The three tuples use relation-type-specific namespaces. Point keys, order-table internal identifiers, radius keys, selectors, nulls, and scalar coefficients are distinct roles. Complete source tuples remain native data. Only independently joined slots supply object references; joined `skamp_ptr` items supply entity and sense operands separately.

**Need.** Obtain complete tuple productions with independently bounded point, order, radius, and incidence joins for each unresolved shape. Preserve scalar coefficients and selector values without treating them as object indices.

**Note.** A tuple position or integer value alone does not establish a namespace. Identical tuple shapes under different relation types do not inherit each other's semantics.

### SP-12. Full-width non-coordinate variable roles

**Question.** Which independently owned scalar roles assign numeric semantics to full-width `var_arr` images outside point-coordinate types `1` and `2`?

**Known.** `creo_prt.md` §5 defines complete nine-byte framing and numeric coordinate roles. Other full-width variable fields retain their exact images as opaque values. A finite IEEE reconstruction alone does not establish a numeric role. Opaque values do not select dimensions or receive equation-derived replacements. Point keys, radius keys, variable ordinals, dimension selectors, and external identifiers remain distinct namespaces.

**Need.** Obtain complete source productions and independent saved-value or authored-property joins for each remaining variable role. Preserve physical framing and source values while those semantics remain unresolved.

### SP-14. Unary point incidence

**Question.** What independent geometric meaning applies to a unary type-one
or type-two sense-zero incidence on a type-5 `segtab` point when `verhor` is
absent or conflicts and no complete type-fourteen symmetry-axis role exists?

**Known.** A unary orientation incidence and agreeing point `verhor` can
persist without type-fourteen axis use. Such records retain the point and two
separate native constraints. A degenerate symmetry axis instead requires its
complete agreeing orientation and type-fourteen conjunction. Missing or
conflicting orientation evidence does not establish an implicit line or origin.
Entity-orientation and point-alignment constraint families are distinct.

**Need.** We must know the constraint to transfer it without inventing an axis.

### SP-16. Other `skamp_ptr` geometry families

**Question.** Which complete owner or incidence join establishes the family of an unresolved projected or solver-only section identity?

**Known.** Entity identifiers are sketch-local. A type-58 owner identifies conic family independently of its stored selector and locus sense. The defined point, endpoint-bearing curve, line, arc, and circle roles require their independent owner or incidence evidence.

**Need.** Establish complete copied-result ownership for projected identities and complete, compatible incidence roles for ownerless identities. A saved result with the same integer in another definition does not establish a join.

**Note.** A missing or ambiguous owner and an unproved locus sense retain native geometry. Family does not follow from the numeric entity identifier or a single center-like sense.

### SP-17. Solver-only external references

**Question.** What complete schema and projection rules handle qualified
external `ref_planes` rows, including a collection qualifier or non-null
`ext_ref_id` or `sub_index`, while preserving their typed model-item namespace?

**Known.** A complete `gsec3d_ptr.ref_planes` row binds its fourth field,
`known_ext_id` or `seg_id`, to the known section-entity identifier. The typed
external target is `(ref_type, plane_id)`. The six-field form adds `sub_index`
and `flip_flag`. Surface and edge references are distinct from table-class,
point-key, relation, and dimension namespaces. Missing local `segtab` geometry
does not prove an external binding. A counted class reference is not a
model-item identifier. The section location frame is a separate projection
input.

**Need.** We must own the complete row schema and any qualifiers before
looking up and projecting the typed model item. Missing or multiple candidate
rows, unknown types, unhandled qualifiers, or an unresolved section frame must
retain native geometry instead of selecting a same-valued datum or class ID.

### SP-18. `exists()` serialized owner graph

**Question.** Which serialized owner records bind a relation context to complete model, feature, component, and section item namespaces?

**Known.** Exact complete-program targets establish existence even when their values are unresolved. A numeric model qualifier selects the zero-based `ref_part_table` row ordinal, not its stored `id`. An `FID_` qualifier selects a feature inside the selected model before the base item selects that feature's parameter or measurement. `CID_` selects a component occurrence by its internal component feature ID. Session and component identities are distinct. Qualifiers resolve from right to left. Section `sd#`, `rsd#`, and `kd#` identities belong to the current section. Parent-model dimensions use `d#` or `rd#`.

**Need.** Bind the current relation owner and referenced documents without filename guessing. Preserve model parameters, feature parameters and measurements, occurrence parameters, and dimension namespaces separately. Resolve name-based features only when unique. Record namespace completeness independently of values. A proved member returns one; a missing member returns zero only in a complete resolved namespace. Missing documents, ambiguous owners, unsupported qualifiers, and incomplete namespaces remain unresolved. Preserve exact program targets before external owner resolution.

### SP-19. External and designated sweep-axis references

**Question.** Which owned fields distinguish an explicitly designated or external revolution axis from the internal centerline, and retain its exact typed model-geometry reference?

**Known.** Absence of `ActDatums` does not select an operation axis. An independently owned placed section supplies the extrusion sweep line. The bounded internal revolution path requires complete tables and one nondegenerate resolved `u=0` line; the nested `gsec3d_ptr.ref_planes` section-entity identifier is not its axis selector. A resolved saved-section frame without an exact operation owner remains insufficient. PTC separately permits designated, geometry, construction, and external linear references; their selection priority does not establish their native field encoding.

**Need.** A positively owned designated/external-axis witness and independent geometry oracle must establish the selector and typed reference without assigning a nearby saved section or interpreting a reference-table identifier as a centerline.

### SP-20. Complete placement-instruction reconciliation

**Question.** Which complete bounded placement-instruction grammar preserves competing nonzero or unresolved offset states before the zero-offset standard-datum placement test?

**Known.** A type-20127 zero-offset instruction with the reference datum as its first geometry operand can select a sketch plane through the exact standard-datum guards in §6. This rule does not select a sweep axis or its sign. Section-plane orientation and section origin require their own geometric evidence. The interactive default orientation convention applies after selecting a sketch plane; it does not supply a missing native sweep direction.

**Need.** Bound all instantiated instructions to their declared class and owning table. Retain each complete row's offset state and reject this fallback when owned instructions disagree or a candidate's offset remains unresolved. Preserve independently proved section transforms and generated-cap-pair placement as separate mechanisms.

### SP-21. Physical row ownership of repeated section-plane records

**Question.** What complete boundary and root-schema grammar safely discovers every physical `AllFeatur` row before decoded feature-namespace admission, and bounds positional section definitions without extending an earlier owner through an omitted row?

**Known.** The bounded class-916 rows with a shared sketch-plane identifier are separate records. In the verified pattern-member form, independent audit rows retain a pattern header and five predecessor-linked members; each member has its own physical row header and positional section record. A shared datum identifier is therefore insufficient to group those records as alternative states. The current fixed-prefix probe and known-feature filter do not establish a general row grammar. A canonical row identifier, a model operation, and a stored section/schema identifier remain separate identities.

**Need.** A complete owned boundary/root grammar and positional section parser must preserve each row range independently of current-feature filtering. Repeated definitions within one genuine owner range, or unresolved `DEPDB_DATA` repeats, require their own selector evidence; first/last order and sketch-plane arithmetic supply none.

### SP-22. Ambiguous sketch-datum parent

**Question.** How does a section select its sketch datum when the generated-datum parent-table remainder is not unique?

**Known.** `creo_prt.md` §6 "`dtm_id_tab [f1|f2] f8 <count> f7 <class> fb e2` is followed by exactly" through `creo_prt.md` §6 "When the sketch plane resolves to a placed plane carrier" define the unique remainder rule and the nested `plane_id` join.

**Need.** We must know the selection rule to place the sketch.

### SP-23. Owned fallback when the orientation reference is parallel

**Question.** Which complete owner-qualified frame or generated carrier places a section when its typed orientation reference is parallel to the sketch normal?

**Known.** A parallel plane normal has no nonzero projection into the sketch plane and supplies no in-plane direction. The existing resolver can use a fully established generated carrier or a unique complete definition local frame, with owner, finite/nonzero axes, and perpendicular-axis guards. The two bounded named reference controls are perpendicular, so they do not establish an authored parallel-case selector. Their direct placement origin differs from their saved local-frame origin; those frames are not interchangeable. `SP-08` retains the distinct placement/transform/saved-result owner problem, and `SP-17` retains qualified typed reference-row ownership.

**Need.** An authored parallel-reference witness with complete frame ownership and independent placed geometry must establish the eligible fallback. Do not select another table member, reinterpret a PSB table class as a model datum, or choose the first/last nearby local frame.

### SP-24. `ActDatums` outline tokens

**Question.** What scalar value does each `5c` and `45` token encode in an `ActDatums` outline?

**Known.** `creo_prt.md` §6 "`ActDatums` stores datum-plane geometry as `act_datum_geoms → srf_array` records. Each section" defines the two-corner outline and its held-coordinate plane rule. Named and positional outlines use the same bounded model-coordinate lane. A `5c` or `45` token consumes one seven-byte coordinate slot without supplying a numeric value.

**Need.** We must know the values to construct nonzero datum offsets and extents.

### SP-25. Other revolution termination selectors

**Question.** How does each rotational-sweep selector other than the full-turn `angle_choice` form define its angular interval?

**Known.** `creo_prt.md` §6 "In a class-916 or class-917 positional feature row, feature form `2` selects a" defines `ea 44 00 00` as a complete 360-degree revolution. Linear sweep extents include one-sided, symmetric, and two-sided spans.

**Need.** We must know the selector semantics to trim a one-sided, symmetric, or two-sided revolution.

### SP-38. Current recipe ownership across creation events

**Question.** Which complete current feature-owned production selects the sweep form when creation-event history disagrees with the current modeling structure?

**Known.** A `created_features` row binds its `feat_id`, `ft_type`, and `icon_name`. A group event can contain distinct child recipes with distinct owners. Class 916 removes material and class 917 adds material; neither class selects extrusion versus revolution. An incompatible creation recipe does not define current form, Boolean effect, or regeneration ancestry. Current parent tables and prototype references remain independent of creation history. Display names, previous-feature identifiers, and pattern membership do not select the current recipe.

**Need.** Establish complete named and positional creation-array and event ownership, including repeated feature identifiers in separate events. Bind the current form independently of first or last audit occurrence, display text, and other child rows. Keep historical fields distinct from current recipe and dependency authority.

### SP-39. Complete outer-entry ownership for simple-drilled dimensions

**Question.** Which complete PSB compound-entry grammar binds each class-911 dimension definition to its current outer feature key, including entries whose declared array count differs from the ordinary keyed-entry count?

**Known.** The current outer key and optional `feat_id / ref_model_info` replay-source reference are separate identities. A copied replay can retain an older source while belonging to a different current key. A complete three-row table still requires the recipe-selected external-ID/type roles. Equal envelopes, names, ordinals, or dimension tuples elsewhere do not establish that owner. A blind depth proved equal to the owned cylinder's axial span ends at its shoulder; the conical point extends beyond it. This independently earns `depth_to_tip = false` only with the cylinder-span and angle evidence.

**Need.** The parser must retain complete current-entry bounds and bind exactly one eligible table before model projection. Unknown compound/replay boundaries must abstain. A missing or invalid owned table must not borrow another feature's equal-radius/depth tuple. Different point-angle controls and envelope-free negatives must preserve both the ownership and Shoulder/Tip boundary.

### SP-41. Native counterbore and countersink additions

**Question.** Which complete feature-owned parameter grammar binds the independent counterbore and countersink additions when entry-profile or transition topology is unavailable?

**Known.** Class-911 `feat_type_ptr.form` distinguishes straight/extruded construction from sketched/revolved construction. It does not declare the detailed entry treatment. `draft_choice` and `misc_choice` are separate fields; a bit position without the complete enclosing production does not identify an entry treatment. Generated conical bottom geometry alone does not identify a countersink. A positively owned entry profile can distinguish a planar shoulder from a conical entry transition.

**Need.** Bind the complete current feature parameter production and its source profile before interpreting packed choices. Preserve counterbore and countersink evidence independently. Bound each outer keyed definition before the next owner; a named sketch prototype boundary does not define the owner interval. Unresolved operands or ambiguous entry transitions must remain unresolved.

### SP-43. Structural `e3` separator

**Question.** What separates a structural `e3` that starts an `ent_tab` replay
row from an `e3` that is the tail byte of a two-byte compact identifier?

**Known.** `creo_prt.md` §8.2 "`segtab` and `ent_tab` compact identifiers may
use `e3` as the tail byte" gives both uses. It gives that the tail is data and
not a row delimiter, and that an `ent_tab` replay row begins after a structural
`e3`.

**Need.** We must separate the two uses to frame `ent_tab` replay rows without
a byte scan.

**Conflict.** `saved_positional_generated_entities` in
`src/feature/definitions.rs:5449-5471` accepts each `e3` byte as a row start
when the following identifier joins a generated segment and an `e2` byte occurs
in the next 24 bytes. The specification does not give this window.

## 4. Topology and appearance

### TP-01. Current feature-to-body binding

**Question.** Which stored regime discriminator and complete current feature-local body references bind an admitted recipe to its input body?

**Known.** The recipe feature and preceding-feature fields in creation-event rows are feature identities, not body selectors. Modern named `body_tree`, `body_array`, and `used_bodies` records contain explicit `body_id` fields; `body_array.first_geom_ptr` supplies a separate material-geometry root. A body count does not identify a feature target. Current generated carriers identify feature outputs rather than input material.

**Need.** Bind complete body identities to the same model owner and current feature scope, then resolve their topology roster independently. Validate the part's material regime before applying a legacy single-material replay rule. Missing, duplicate, unresolved, or conflicting target identities must remain unresolved; neither chronology nor the first or sole decoded body selects a modern target. Creation history alone does not establish the current recipe or its body operands.

### TP-02. Cross-section parameter framing and contour ownership

**Question.** Which complete parameter grammar distinguishes structural row
closes from delimiter-like bytes inside opaque and scalar parameter data?

**Known.** `Sld_Xsections.xsec_geom.crv_array` has six-reference
`[0, P, 0, N, S, 0]` tails. Its plane, successor, and reference-surface
identities belong to the cross-section and generating-feature namespaces.
They do not identify final-solid vertices, faces, or loops.

**Need.** Bound every parameter form and each row close within its owning
array. Require unique same-feature surface and curve targets and a complete
successor permutation before constructing section contours. Preserve all
independently closed cycles; a feature does not imply one contour.

**Note.** A tail split alone does not establish the complete row grammar or
prove that a section contour is current material topology.

### TP-03. Remaining contour roles and incomplete joins

**Question.** Which roles do contour values `00`, `02`, and `03` encode, and
which source-owned reference resolves a contour whose head does not join one
unique closed loop in its topology namespace?

**Known.** Positional `trv = 01` identifies an outer boundary and `trv = f6`
identifies an inner boundary. The preceding complete two-byte curve-header
reference selects the unique same-surface closed successor loop containing
that header. A complete classification requires a contour-to-loop bijection.
The role is independent of chain order and surface-row orientation. The
remaining grammar values do not establish a role. Incomplete or ambiguous
joins do not establish loop ownership. Multiple outer loops require the
face-instance partition described by TP-06.

**Need.** We must know the remaining role values and owner-qualified joins to
classify loops that the complete stored role mapping does not cover.

### TP-04. Additional vertex-coordinate selectors

**Question.** Which source-owned coordinate witness selects one point for a
B-rep vertex orbit when directed pcurve endpoints, intrinsic edge endpoints,
and incident analytic carrier constraints leave several candidates or none?

**Known.** `crv_array` face and successor references define half-edge vertex
orbits. The orbit labels are derived from that graph; the six-reference suffix
does not store vertex identifiers or XYZ tuples. Complete agreeing directed
pcurve paths bind their mapped endpoints to the two oriented start orbits.
Intrinsic edge endpoints and incident carrier intersections supply further
constraints. A point requires one consistent candidate. Sketch
`vert_tab.vertex_id` belongs to a separate trim namespace and does not bind a
B-rep orbit. The defined suffix grammar does not establish a direct stored
vertex-coordinate fallback in another record family.

**Need.** We must know any additional owner-qualified coordinate selector to
place an orbit whose existing evidence does not select one point.

### TP-05. General rowless face-use binding

**Question.** Which fields bind a rowless face-use reference to its face, loop, edge, and orientation outside the round-feature rowless-cylinder table?

**Known.** `creo_prt.md` §6 "Classes 913" through `creo_prt.md` §6 "For a class-913 cylindrical slot fillet, the first two `geoms_affected`" define the round-feature rowless-cylinder special case.

**Need.** We must know the general binding to construct those face uses.

### TP-06. Shared-surface face partition

**Question.** Which field partitions uses of one surface reference into separate face instances?

**Known.** A surface carrier identity does not by itself identify one bounded face instance.

**Need.** We must know the partition to prevent distinct faces from collapsing into one face.

### TP-07. `lo_restore.direction`

**Question.** What does the `lo_restore.direction` compact integer refer to, and how does it control loop traversal?

**Known.** `creo_prt.md` §6 "Within an `AllFeatur` `lo_restore` body, named-record type-one fields" states that it belongs to a loop-restoration edge record and is not a sweep direction or extent.

**Need.** We must know its referent and sense to restore the loop order.

### TP-08. `lo_restore.direction2`

**Question.** What does the `lo_restore.direction2` compact integer refer to, and how does it control loop traversal?

**Known.** `creo_prt.md` §6 "Within an `AllFeatur` `lo_restore` body, named-record type-one fields" states that it belongs to a loop-restoration edge record and is not a sweep direction or extent.

**Need.** We must know its referent and sense to restore the loop order.

### TP-09. Required `lo_hist` fields

**Question.** What semantic role does each of the four required `lo_hist` fields have?

**Known.** `creo_prt.md` §6 "A named `lo_id_tab_ptr` table can be followed in the same feature row by" defines the six-entry stored loop-history frame and preserves all field identities.

**Need.** We must know the roles to reconstruct feature-local loop history.

### TP-10. Optional `lo_hist` field

**Question.** What semantic role does the optional final `lo_hist` field have?

**Known.** The decoder preserves the optional field separately from the four required fields.

**Need.** We must know the role to apply the complete loop-history record.

### TP-11. Loop-to-boundary-surface join

**Question.** Which field joins a feature-local loop identifier to its boundary-surface curve network?

**Known.** Loop-history identifiers and surface and curve identifiers occupy separate namespaces.

**Need.** We must know the join to bind the restored loop to geometric edges.

### TP-12. Shell-to-body assignment

**Question.** Which byte-backed relation assigns a shell to a body when face-adjacency components and body-count fields disagree?

**Known.** Face adjacency gives connected shell components. A body-count field gives the expected body cardinality but not shell ownership. Legacy ASCII admission can identify eligible visible faces and retain a single admitted component; multiple admitted components still lack a shell-to-body join.

**Need.** We must know the relation to construct the correct body membership.

### TP-13. Element-color side and instance ownership

**Question.** Which serialized slot convention binds the two `color_inds`
values to an oriented model-item side, and which owner-qualified reference
resolves item identifiers outside the established topology-reference match?

**Known.** Complete two-empty-property `element_colors` rows store one item
identifier, two ordered color indices, and two empty `surfprop_ptr` bodies.
The row contains no contour, loop, boundary-role, or bounded-face selector.
Exact item identifiers can occur in topology face-reference pairs without a
materialized surface row. Such a reference can support several bounded faces.
PTC's side-appearance API accepts a model item and side zero along the normal
or side one on the other side; that API alone does not establish serialized
array order. A selected-set graph with the same counts and degrees as an
exported appearance set does not prove unique individual face identity under
symmetry. Nonempty property pointers are outside the empty-body grammar.

**Need.** We must know the directional slot convention, additional item
namespaces, and complete carrier-to-instance ownership before transferring
these appearances onto bounded face sides.

### TP-14. Unresolved `NeuPrtSld` appearance references

**Question.** Which additional owner-qualified `NeuPrtSld` records, if any,
map unresolved appearance item identifiers to their native model-item class,
carrier, and oriented bounded-face uses?

**Known.** Complete two-empty-property `element_colors` rows contain one item
identifier and two ordered color/property slots without a contour or bounded
face selector. Exact topology-reference matches do not require a materialized
surface row. Other fully decoded tables can retain identifiers absent from
all tested visible and nonvisible surface, curve, topology-reference, and
feature-row sets. Those nonmatches establish unresolved references; they do
not prove a different model-item class or exclude an unmaterialized or stale
reference. The public appearance API supports multiple model-item classes,
but does not identify the class of an unmatched serialized value. Slot-to-side
ordering and nonempty property pointees remain separate unresolved contracts.

**Need.** We must know those identity and side contracts before promoting
unresolved native appearances onto model faces. A nearest ID, table ordinal,
or unique exported graph shape does not supply that join.

### TP-15. Display side and occurrence ownership

**Question.** Which serialized convention assigns ASCII display color and
placement slots to oriented surface sides, and which reference supplies an
assembly occurrence owner or a nonempty placement-property pointee?

**Known.** A complete uncompressed ASCII `surf_ind_tab` record stores one
`surf_id`, two ordered `surf_color_inds`, and two `placement_ptr` entries.
Declaration numbers and object depth are schema structure, not the carrier
identity. Exact stored IDs can match current topology references without a
materialized surface row. The complete two-empty-placement form has no
additional contour, loop, or bounded-face selector. Equal color pairs do not
identify slot-to-side direction. A topology-reference match does not prove a
complete bounded-face partition, and a coincident integer in another model
does not establish an assembly occurrence. Nonempty placements and compressed
display bodies require their own grammar.

**Need.** We must know those side, occurrence, and property contracts before
transferring display metadata onto final bounded face uses.

### TP-16. Unresolved RGB constants and appearance projection

**Question.** Which RGB-lane evidence fixes `0d` and `18`, and which owned
row grammar binds the channels to an appearance and its model-item uses?

**Known.** `creo_prt.md` §1.2 defines RGB `0f = 1` and bounded predecessor
replay within three channels. The non-RGB transmission field stores
`18 = 0`; that field does not define RGB `18`. Complete `NeuPrtSld` bytes
are retained without a typed `color_tab` parser or appearance-binding
projection. Explicit positive RGB values remain separate from unresolved
compact constants.

**Need.** Establish each unresolved constant with independently paired RGB
definitions or controlled RGB changes. Define complete named and positional
row ownership and model-item binding before emitting colors and their uses.

**Conflict.** STEP green `.4` covariance does not identify native RGB `0d`.
A component value in another lane does not establish the RGB value. A STEP
color fit cannot replace the native value without an independent row identity.

### TP-17. Other appearance component lanes

**Question.** What scalar value does each undefined token in a non-RGB appearance component lane encode?

**Known.** `creo_prt.md` §1.2 defines field-qualified `diffuse:0f=1`, `transmission:18=0`, and the highlight-vector repeat control. Those meanings do not transfer to other component fields. Complete `NeuPrtSld` bytes preserve the remaining scalar images without establishing their values, appearance ownership, or surface side.

**Need.** Define complete owned appearance rows, retain unresolved component bodies, and establish each value independently with an exact field/component oracle. API ranges and values in another lane do not identify a token. Keep appearance binding and side selection separate from scalar decoding.

### TP-18. Complete modification-audit event grammar

**Question.** Which complete named and positional grammar recovers each
`mtrack_record_ptr` action payload and its field ownership?

**Known.** `MdlStatus.mod_track_data.rec_arr` is a modification audit trail.
The timestamp's `tm_year` precedes `item_name`; printable year bytes are not
name-state prefixes. Repeated same-ID event names do not select a current
feature operation. Whole physical `MdlStatus` bytes can be retained under
their named section owner without projecting an event as current.

**Need.** Establish complete bounds and field types for each action-specific
payload and its references. Current operations require separate current-model
ownership evidence; audit names and order do not supply it.

**Conflict.** Interpreting an audit timestamp's final scalar as a name-state
prefix contradicts its declared field ownership.

**Note.** The named prototype and compact six-scalar timestamp form do not
define the complete heterogeneous event union.

### TP-19. `lo_array` roster semantics

**Question.** Which fields in a positional `lo_array` row bind its native
roster entry to a face, contour chain, curve header, and traversal orientation?

**Known.** `creo_prt.md` §3.5 defines the `lo_array` frame header, named
prototype fields, seven-field positional prefix, and bounded row body. The
decoder retains complete frame and row bytes without assigning neutral loop or
face meaning.

**Need.** We must know the joins before `lo_array` rows can contribute to
neutral loop ownership or face admission.

## 5. Packed persistence data

### PP-01. Packed `VisibGeom` records

**Question.** What geometry record grammar does packed `VisibGeom` use outside the defined PSB rows?

**Known.** `creo_prt.md` §3 "`VisibGeom`" defines material `VisibGeom` namespaces that contain PSB `srf_array` and `crv_array` rows.

**Need.** We must know the packed grammar to construct its remaining geometry.

### PP-02. `SolidPrimdata` positional strip replay

**Question.** Which complete positional class-84 body grammars and primitive scalar forms define each strip set?

**Known.** `creo_prt.md` §8.4 defines record-local cumulative intervals and winding reset. A strip does not continue across records. Compact and array-framed positional replays retain cumulative boundaries, fixed attribute fields, and XYZ position arrays. The position-only profile and the interleaved normal-position profile have different attribute tuples and size accounting. Neither `obj_type=84` nor `num_offsets` supplies the strip count. A bounded nine-slot position body can retain an unresolved three-byte scalar without defining its numeric value.

**Need.** Complete physical framing must establish each positional cumulative-array extent and position count, including multiple-strip forms, before numeric geometry admission. Unresolved primitive scalar values and display placement require independent definitions.

### PP-03. Complete positional primitive-owner framing

**Question.** Which complete nested field trees delimit positional `pgl_segref` rows and their primitive containers, and which independent ownership rule associates a resolved display object with analytical geometry?

**Known.** `creo_prt.md` §8.4 defines the named row's display reference and its table/class/ordinal domain. The primitive decoder has no typed persistent-reference resolver or owner attachment. The bounded raw reference permutation and identifier correspondences do not prove complete nested row boundaries or a general face join.

**Need.** Establish complete positional row and nested-array boundaries before primitives inherit display references. Resolve missing or ambiguous tables, handles and row ownership conservatively. Prove a unique same-model geometry association separately; reference-like bytes inside opaque primitive payloads and equal numeric identifiers do not supply it.

### PP-04. Expanded `SolidPersistTable` row bodies

**Question.** Which complete named-prototype field trees bound each persistent object row and its nested arrays, including polymorphic shader bodies?

**Known.** `creo_prt.md` §8.8 defines the bounded table/class/ordinal reference domain, named row zero, handle-name variants, and separate segment identifiers. Handle class differs from concrete row type. The container expansion retains the payload; the decoder has no typed PGL object graph or persistent-reference resolver. Recognized common headers alone do not prove full row boundaries.

**Need.** Establish complete field-tree grammars for the object classes and host-prefix variants, including nested reference ownership. Preserve ordered rows and duplicate stored IDs. Missing or ambiguous table/class selection, absent GPU membership, and unresolved model/occurrence/display placement must remain explicit. Original nonzero or multiple-table cases and the historical population totals remain unverified; a private bounded selector is not a production parser.

### PP-05. Physical `DEPDB_DATA` semantic ownership

**Question.** Which complete physical persistence bodies, beyond the observed empty roots, establish feature, section, unit or model-metadata ownership?

**Known.** Three independently indexed originals contain complete empty physical `DEPDB_DATA` roots. Their surface, curve, section and recipe labels belong to other indexed sections. This establishes those three boundaries, not a population-wide absence result or a replacement owner for every implemented value grammar. The visible-geometry and namespace-census readers do not borrow physical `DEPDB_DATA` when `VisibGeom` is absent or sparse; newly refused geometry-looking sections retain their complete physical bytes with an explicit untyped-owner warning. Failed expansion retains its existing precedence.

**Need.** Establish source-owned complete grammars and audit the remaining physical feature/operation, section, model-name and unit readers before further semantic admission or quarantine. Preserve unknown physical variants and original offsets. Authored composition fixtures and an encoding-family name are not native ownership evidence; fixture relocation requires a proved producer grammar.

### PP-06. Compressed `DispDataTable` substrate

**Question.** Which encoding and initial state expand the `DispDataTable`
bodies whose `1f 9d 10` prefix is followed by invalid ordinary LZW codes?

**Known.** A non-block `1f 9d 10` stream starts with a nine-bit literal code
and dictionary slots `0..255`. A first code of 480 violates that state; a later
code beyond the next free dictionary slot also violates it. These failures can
occur in complete TOC-bounded bodies whose two stored checksum lanes agree.
Successful ordinary streams in the same container do not select a different
dictionary for such a body. The stored bytes remain opaque.

**Need.** Define the method selector and complete decoding state, then verify
the declared output length and a fully consumed record grammar with an
independent decoder. A partial output, valid magic prefix, or printable run
does not establish the substrate.

### PP-07. Compressed `DispDataTable` geometry binding

**Question.** Which fields bind decompressed `DispDataTable` rows to model geometry?

**Known.** The compressed stream contains display data in a namespace separate from material face instances.

**Need.** We must know the binding to apply display data to the correct entities.

### PP-09. Neutral family-table semantics

**Question.** Which family-table item types represent neutral parameters, dimensions, feature states, active instances, and configuration values?

**Known.** `creo_prt.md` §8.3 defines the legacy object graph, ordered `items` and `instances` arrays, ordinal value join, complete row fields, and typed value forms 50, 51, and 52. The decoder retains a complete joined table in the native `configuration_driver_tables` arena.

**Need.** We must map the complete table columns and instance state to neutral configuration, parameter, feature-state, and active-configuration records without assigning a meaning to an item type or visibility flag that the source does not establish.

### PP-13. Legacy persistence bodies

**Question.** What type and value grammar does each legacy ASCII `@<name>`
field declaration select, and how do its object references and arrays compose
the geometry, topology, and design-history graphs? What record grammar applies
to named legacy sections that do not begin with an attribute declaration?

**Known.** `creo_prt.md` §1 defines the complete legacy ASCII layout
discriminator, its outer `P_OBJECT` boundary, its decimal schema token, its
product-release banner forms, and its monolithic and named-section forms. The
named-section directory defines banner-relative offsets and stored extents.
Attribute declarations and value rows use section-local identifiers; an
immediately following `$` row continues a value payload.
The unique type-10 `principal_sys_units` scalar selects either millimeter or
inch coordinate lengths; inch lengths scale by `25.4` to canonical
millimeters.
Type 1 selects signed decimal 32-bit integer scalars and dimensioned run-length
arrays. Type 2 selects finite IEEE-754 binary64 scalars and dimensioned
run-length arrays with the compact hexadecimal grammar in `creo_prt.md` §1.
Type 0 selects null, arrow, inline, and dimensioned-array object nodes; row
depth supplies their scoped ownership tree.
Type 10 selects null and byte-string scalars and direct-row arrays. The first
array extent gives the number of string rows; later extents do not multiply
that count. Valid UTF-8 strings transfer as text, and other encodings retain
their exact bytes.
Type 3 selects nullable byte-string scalars. Type 4 selects byte-string scalars
without a null token. Neither type uses continuation rows.
Type 6 uses the type-2 compact-real scalar and array grammar. Types 5, 7, 9,
and 11 use unsigned decimal 32-bit scalars and dimension-complete run-length
arrays; their one-element arrays can store the element in a direct child row.
In `Sld_VisGeom.active_geom`, a complete direct-child `crv_array` object has
one `crv_id`, `type`, `feat_id`, signed two-element `crv_pnt_dir`, two
`crv_hdr_geom_ptr` face references, and two `next_crv_hdr_ptr` successor
references. A complete `[N,4]` `crv_pnt_arr` type-2 array uses its first and
last rows as the two endpoint pairs in the adjacent face charts. These rows
join the native half-edge and face-loop graph when all carrier and vertex
admission invariants hold.

**Need.** We must know the semantic axis order of multidimensional type-2
arrays outside the settled geometry records, the character-set selection for
non-UTF-8 type-10 strings, type-10 continuation semantics, remaining geometry
and design-history graph joins, and non-attribute section grammar to
transfer the rest of legacy persistence.
