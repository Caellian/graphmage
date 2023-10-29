Group is a [node](./node.md) that contains a collection of nodes to be drawn
together.

Root of the document is treated as contents of a group declaration.

## Layout

Layouts provide automatic placement of nodes within group bounds according to
some use-specific rules.

During layouting, any forces that affect the node layout are applied to a group
but not the inner nodes. Layout engines are allowed to dictate positions of
grouped nodes in order to reduce edge lengths and overlap.

In case a group uses a different layout engine than the outer scope, group
contents are layouted first, separate from the rest of the document and then the
outer engine treats the group as a single node.

Node positions are non-deterministic when different layouts are used and it's
left up to individual layout engines to provide such a guarantee if it's
possible.

## Properties

Groups support all [node properties](./node.md#properties).

| Property | Description | Inherited | Default |
| :------: | ----------- | :-------: | ------- |
| `layout` |             |    Yes    |         |
