<script lang="ts">
    const images: Record<
        | "kd"
        | "qd"
        | "rd"
        | "bd"
        | "nd"
        | "pd"
        | "kl"
        | "ql"
        | "rl"
        | "bl"
        | "nl"
        | "pl",
        string
    > = {
        kd: "https://upload.wikimedia.org/wikipedia/commons/f/f0/Chess_kdt45.svg",
        qd: "https://upload.wikimedia.org/wikipedia/commons/4/47/Chess_qdt45.svg",
        rd: "https://upload.wikimedia.org/wikipedia/commons/f/ff/Chess_rdt45.svg",
        bd: "https://upload.wikimedia.org/wikipedia/commons/9/98/Chess_bdt45.svg",
        nd: "https://upload.wikimedia.org/wikipedia/commons/e/ef/Chess_ndt45.svg",
        pd: "https://upload.wikimedia.org/wikipedia/commons/c/c7/Chess_pdt45.svg",
        kl: "https://upload.wikimedia.org/wikipedia/commons/4/42/Chess_klt45.svg",
        ql: "https://upload.wikimedia.org/wikipedia/commons/1/15/Chess_qlt45.svg",
        rl: "https://upload.wikimedia.org/wikipedia/commons/7/72/Chess_rlt45.svg",
        bl: "https://upload.wikimedia.org/wikipedia/commons/b/b1/Chess_blt45.svg",
        nl: "https://upload.wikimedia.org/wikipedia/commons/7/70/Chess_nlt45.svg",
        pl: "https://upload.wikimedia.org/wikipedia/commons/4/45/Chess_plt45.svg",
    };

    function string_to_image(str: keyof typeof images): string {
        return images[str];
    }

    // array of array for boards
    let board: Array<Array<string>> = Array(8).fill(Array(8).fill(""));

    // setup board
    board[0] = ["rd", "nd", "bd", "qd", "kd", "bd", "nd", "rd"];
    board[1] = ["pd", "pd", "pd", "pd", "pd", "pd", "pd", "pd"];
    board[6] = ["pl", "pl", "pl", "pl", "pl", "pl", "pl", "pl"];
    board[7] = ["rl", "nl", "bl", "ql", "kl", "bl", "nl", "rl"];
</script>

<main
    class="flex flex-row items-center justify-center h-screen overflow-clip rounded-md"
>
    <ol class="grid grid-flow-col">
        {#each board as row, i}
            <ol class="">
                {#each row as cell, j}
                    <li
                        class={`w-20 h-20 ${(i + j) % 2 == 0 ? "bg-gray-300" : "bg-gray-100"}`}
                    >
                        {#if board[i][j] != ""}
                            <img src={string_to_image(board[i][j])} alt="" />
                        {/if}
                    </li>
                {/each}
            </ol>
        {/each}
    </ol>
</main>
