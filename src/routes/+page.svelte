<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let current_fen =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    const board = Array(8).fill(Array(8).fill(null));
</script>

<div class="w-full h-full flex items-center justify-center mt-32">
    <ol class="grid grid-rows-8 rounded-md overflow-clip">
        {#each board as row, i}
            <div class="flex">
                {#each row as square, j}
                    <div
                        class={`h-20 w-20 flex items-center justify-center ${
                            (i + j) % 2 === 0 ? "bg-gray-300" : "bg-gray-500"
                        }`}
                    >
                    </div>
                {/each}
            </div>
        {/each}
    </ol>
</div>
