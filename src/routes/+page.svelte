<script lang="ts">
    import { string_to_image, type Piece } from "$lib";
    // array of array for boards
    let board: Array<Array<Piece>> = Array(8).fill(Array(8).fill(""));
    // setup board
    board[0] = ["rd", "nd", "bd", "qd", "kd", "bd", "nd", "rd"];
    board[1] = ["pd", "pd", "pd", "pd", "pd", "pd", "pd", "pd"];
    board[6] = ["pl", "pl", "pl", "pl", "pl", "pl", "pl", "pl"];
    board[7] = ["rl", "nl", "bl", "ql", "kl", "bl", "nl", "rl"];
</script>

<main
    class="flex flex-row items-center justify-center h-screen overflow-clip bg-gray-900"
>
    <ol class="grid grid-flow-col grid-rows-8 rounded-md overflow-clip select-none">
        {#each board as row, i}
            <ol class="grid grid-flow-row grid-cols-8">
                {#each row as cell, j}
                    <li
                        class={`w-20 h-20 ${(i + j) % 2 == 0 ? "bg-gray-300" : "bg-gray-100"}`}
                    >
                        {#if cell}
                            <span
                                class="flex justify-center w-full h-full items-center"
                            >
                                <img
                                    src={string_to_image(cell)}
                                    alt=""
                                    class="w-3/4 h-3/4"
                                />
                            </span>
                        {/if}
                    </li>
                {/each}
            </ol>
        {/each}
    </ol>
</main>
