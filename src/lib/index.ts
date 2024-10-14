export type Piece = "kd" | "qd" | "rd" | "bd" | "nd" | "pd" | "kl" | "ql" | "rl" | "bl" | "nl" | "pl" | "";

export const images: Record<Piece, string> = {
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
    "": "",
};

export function string_to_image(str: Piece): string {
    return images[str];
}
