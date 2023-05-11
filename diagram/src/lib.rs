/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    lib.rs
@brief   Diagram library
 */

//a Documentation
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
/*!
# Diagram library

The diagram library provides the support for creating styled
 diagrams, usually using a markup language to describe a diagram and
 its styling, and generating an SVG output.

The diagrams use a boxed layout model, similar to web pages - and the
 styling is similar to cascading stylesheets.

## Diagram elements

The diagram is made up of elements, which are either leaf elements
(shapes, text) or collections of elements (groups, layouts).

Elements are fitted with rectangles, which may be rendered with
padding, border and margin, with a background within the border
possibly filled. The content of the element is rendered within this
(filled) border, with an optional scaling and arbitrary rotation.

Elements are rendered in the order in which they appear within the
diagram specification; hence later elements in the diagram are drawn
over earlier elements.

### Leaf element types

The rendered elements in a drawing are currently shapes and
text.

Shapes are regular polygons or regular stars, circles or
ellipses; they may be filled with a solid color, and they may have
their edges drawn; they may have rounded corners.

Text elements are multiple lines of text

Path elements are single or multiple line segments defined by
coordinates relative to the box they are placed within.

### Group element

The group element is purely a collection of elements; it will be laid
out, but the contents themselves specify how they will be laid out in
the parent of the group. The group element, though, will have an
associated border which can be rendered if required; hence a group is
used to group elements, and sometimes to provide an additional empty
(optionally filled) border.

### Layout element

The layout element is another collection of elements, and it is
fundamental to the operation of the diagram. It provides both a grid
layout mechanism and a placement mechanism, for its contents. The
contents of the layout element know nothing of the layout of the
diagram outside of the layout element.

Gridded layout uses row numbers and column numbers, and spans, and
styling to specify how the rows and columns are laid out; each leaf
element is laid out within a cell occupying a span of rows and a span
of columns.

### Path element

A path element consist of lines (only) at present.

They may be rounded, and closed

If closed *and* the path does not end up at the same point at which it
starts then an extra point, copied from the first point, is added to
the path.

### Rect element - a *shape*

Rectangles are polygonan shape with four vertices, a width and a
height; the height defaults to the width.

### Circle element - a *shape*

Circles are polygonan shape with zero vertices, a width and a height;
the height defaults to the width. This permits them, in fact, to be
ellipses. They are actually approximations using four cubic Bezier
curves - but the approximation is to within about .15% at worst.

### Polygon element - a *shape*

Polygon elements have any number of vertices (of three or more), and
have a *width* that is actually the distance of the points from the
center of the polygon. The polygon is a regular polygon, with points
at angles of multiples of 360/N offset by half.

The polygons may be *stellate*d, where an additional N points are
placed between each of the polygon points at a distance of *stellate*
from the centre.

## Definitions and Uses

A diagram may contain definitions of collections of elements, such as
(for example) a queue may be defined to be a layout of four rounded
rectangles. This definition may be given, for example an 'id' of 'Queue'.

```text
#defs
##rect id=Queue stroke-color=red round=1. width=10. height=5.
```

The diagram contents may then *use* this definition, for example with:

```text
#use ref=Queue grid=1,1
#use ref=Queue grid=2,1
```

This instances the four rounded rectangles twice, and so there will be
two `Queue`s in the diagram.

## Styling

The structure of a diagram should be defined by the diagram
elements. This implies that the layouts that form the diagram are
defined, and the contents of each layout are defined.

The diagram *may* be styled within its description too: styles (such as
fill colors, line widths, and so on) may be defined using attributes
in a markup diagram, for example.

Diagram elements *should* be provided with 'id's and 'class'
attributes; the 'class' is a list of tokens separated by whitespace -
and hence an element may be considered to be in *many* classes.

A set of stylings and style rules can then be applied to a diagram, to
permit the diagram styling to be developed *independently of its
structure*. This is the same operation as for web pages with CSS
(cascading stylesheets). It is normal for 90% of the time spent on a
diagram to be playing with the styling, compared with 10% developing
its structure.

## Layout types

A grid layout uses a specification for each cell that participates
 in the layout, using a pair of cell start/end indications for the
 cell. The X and Y are laid out independently. All the cells within
 the grid are defined, and then styled (given style values from the
 stylesheet). Then the cells contents are interrogated to determine
 their *desired size*, to which scaling, rotation, padding, border
 and margin are added. The grid layout uses the cell start/end
 indications and the desired size to determine the demands of the
 cell on the X and Y dimensions of the grid layout. This produces a
 basic layout for each cell grid X and Y value mapping to a desired
 grid X and Y value; and the grid therefore has a desired size.

The grid layout will eventually be asked to be laid out in a real
 geometry for the diagram.  At this point the real geometry may be
 larger than the desired, in which case the grid may be expanded if
 required by the styles.

To permit the styling of the layout the grid may also be provided
 with minimum sizes for cell ranges, in the styling of the `layout`
 element. These are lists of <int> (<float> <int>)*; the ints
 should be in increasing order, and they specify the cell
 boundaries; the floats are the minimum size between its two
 neighboring float boundaries.

An example layout could be just two elements:

```text
#layout ##circle id=a grid=1,1 ##circle id=b grid=2,1
```

This specifies two shapes, one at grid cell (1,1,2,2) (there is a
 default span of one row and one column), and the second at grid
 cell (2,1,3,2). The grid therefore has in the X dimension cell
 boundaries at 1, 2 and 3; in Y it just has 1 and 2 (i.e. a single row).

These two shapes will be laid out, therefore, in a single row,
 using the sizes required by the shapes. The row will be tall
 enough for the taller of the two shapes.

If the shapes are of different size, but the desire is to have the
 cells be the same width of 50 (provided the shapes are smaller
 than that) then one can provide the minimum sizes:

```text
#layout minx=1,50.,2,50.,3 ##circle id=a grid=1,1 ##circle id=b grid=2,1
```

Now the minimum width (X dimension) between cell 1 and cell 2 will
 be 50. units, and the same is required between cells 2 and 3.

# Example diagrams

A simple first example diagram consists of four shapes laid out in a 2-by-2 grid:

```text
#diagram
##polygon vertices=3 grid=1,1 fill=blue width=10 stroke=yellow strokewidth=1
##polygon vertices=4 grid=1,2 fill=pink width=10
##polygon vertices=3 grid=2,1 fill=blue width=10 stellate=8 stroke=yellow strokewidth=1
##polygon vertices=4 grid=2,2 fill=pink width=10 stellate=8
```

# Status

This is very much a 0.1 version.

The diagram provides very simple text and polygons with the grid-based
layout. It supports a first cut of the stylesheet mechanism.

Diagrams written for this version may need to change for version 0.3.
Path support for bezier curves and closed paths will be added.

## Upcoming changes

Nothing imminent

## Upcoming additions

### Style rules

The initial revision of style rules applies them in style-sheet order;
last style that matches is applied. This will change to highest
priority rule match - with each rule allowed to specify a priority,
the default being its depth (supplying a 'longest rule wins' by
default).

### Label elements

A new element *label* need to be added, which has no desired geometry,
but is an element that is attached to a reference point that is on a
path or object, and which has an offset from that reference point to
the contents of the label.

The label is rendered with its contents in a box of its desired
geometry, and with a rendering of the callout from the reference point
to the offset point as, for example, a line, or a callout bubble.

An example would be to label a path on a diagram.

### Connector elements

A new element *connector* needs to be added, which has no desired
geometry. It has attributes such as stroke, stroke-width, markers,
round, etc. It will have a programmatic description of the points that
it defines; it may be defined to be Manhattan style; it may be defined
to have start and end points that are normals to their attachment
points.

The purpose of the path element is to provide for connectors between
blocks on a diagram.

### Programmatic elements

A simple byte-code interpreter is required that can interpret a
byte-code similar to that used for Ocaml, using a stack and objects
that are reference counted. The primary types supported are 63-bit
integers and objects. The interpreter performs no type-checking
itself. The interpreter is expected to be short-lived.

The interpreter runs a code a program which provides operations to
create a new stack frame of size N, do arithmetic operations on
integer objects on the stack, and invoke functions based on the stack
contents.

The purpose is to support a simple compiler that implements any
type-checking required, which generates the byte-code.

The functions supported by the use of the interpreter in the diagram
rendered will enable access to geometry and style attributes of
elements, and to provide for the generation of path geometries for
objects such as *path* elements and *connector*s.

### Style propagation

The design of the style system is based on inheriting the value of
styles from the parent objects where this is specified. For example,
one ought to be able to specify at a *layout* element that the
contents should all have a particular border style, unless those
elements override that.

This inheritance is not yet supported.

### Markup 'include's

The markup should support '#include' tags that permit inclusion of
other markup files as complete elements within the markup.

This will provide for libraries and stylesheets to be specified within
a document by default.

### Style rule resolution

A style *rule* specifies a *style* attribute that is a reference to a *style* with that *id*.

The resolution of this id to a style currently occurs at the point in
the markup that the rule is spotted. This precludes the use of
stylesheets, and basically is wrong.

The resolution of style names to styles must be performed at a
stylesheet name resolution point.

## Render binary additions

The command-line tool for rendering diagrams must support a diagram
description with multiple markup files; one for the diagram, others
for stylesheets particular to rendering.

# Open issues

border should be  border-width
borderwidth should be the color

!*/

//a Imports and exports
mod constants;
mod diagram;
mod diagram_ml;
mod grid;
mod layout;

pub(crate) use grid::{GridData, GridPlacement};

pub(crate) use self::diagram::{DiagramContents, StyleRule, StyleSheet};
pub(crate) use layout::{Layout, LayoutBox, LayoutRecord};

pub use self::diagram::{Diagram, DiagramDescriptor};
pub use self::diagram::{GenerateSvg, Svg};
pub use diagram_ml::{DiagramML, MLErrorList};
