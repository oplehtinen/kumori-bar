import { serve } from "@hono/node-server";
import { Hono } from "hono";
import winplayer, { type Status, type Position } from "@innei/winplayer-rs/emitter";

const app = new Hono();
const PORT = 6969;

let mediaStatus: Status;
let mediaPos: Position;

app.get("/", (ctx) => {

    return ctx.json({
        status: mediaStatus,
        position: mediaPos,
    });
})

serve(app, async () => {
    const playerManager = await winplayer();
    if (playerManager) {
        playerManager.on("MediaPropertiesChanged", (status: Status) => {
            mediaStatus = status;
            console.log("MediaPropertiesChanged", status);
        });

        playerManager.on("PlaybackInfoChanged", (status: Status) => {
            mediaStatus = status;
            console.log("PlaybackInfoChanged", status);
        });

        playerManager.on("TimelinePropertiesChanged", (position: Position) => {
            mediaPos = position;
            console.log("TimelinePropertiesChanged", position);
        });
    }
    console.log(`Server is running on port ${PORT}`);
}).listen(PORT);