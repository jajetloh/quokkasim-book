# 4. Examples

A number of example simulations exist to act as a reference for how to apply the concepts covered in this guide, in practice:

<table>
<thead>
<tr>
    <td>Example Name</td>
    <td>Description</td>
    <td>Concepts Demonstrated</td>
</tr>
</thead>
<tbody>
<tr>
    <td><a href="https://github.com/jajetloh/quokkasim/blob/main/quokkasim_examples/src/bin/material_blending.rs">material_blending.rs</a></td>
    <td>Model of a stockyard with Reclaimers to remove material from multiple stockpiles, and a Stacker to add material to multiple stockpiles.</td>
    <td class="badge-container">
        <span class="badge">Pre-built components</span>
        <span class="badge">Vector Resource (Vector3)</span>
        <span class="badge">Splitters and Combiners</span>
    </td>
</tr>
<tr>
    <td><a href="https://github.com/jajetloh/quokkasim/blob/main/quokkasim_examples/src/bin/discrete_queue.rs">discrete_queue.rs</a></td>
    <td>Basic example demonstrating use of a Sequence resource of Strings. Processes and stocks are arranged in infinite loop.</td>
    <td class="badge-container">
        <span class="badge">Pre-built components</span>
        <span class="badge">Sequence Resource (String)</span>
        <span class="badge">Distribution Sampling</span>
    </td>
</tr>
<tr>
    <td><a href="https://github.com/jajetloh/quokkasim/blob/main/quokkasim_examples/src/bin/iron_ore.rs">iron_ore.rs</a></td>
    <td>Processes involving an Iron Ore resource, which tracks elemental mass composition, and composition by mineral types.</td>
    <td class="badge-container">
        <span class="badge">Custom vector resource</span>
        <span class="badge">Distribution Sampling</span>
    </td>
</tr>
</tbody>
</table>