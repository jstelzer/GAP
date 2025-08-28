import asyncio, json, websockets, time

async def run():
    async with websockets.connect("ws://127.0.0.1:7777") as ws:
        await ws.send(json.dumps({"type":"hello","version":"0.2.0","agent":"test"}))
        async for msg in ws:
            m = json.loads(msg)
            if m.get("type")=="state":
                p = m["data"]["player"]["pos"]
                tx, ty = p[0]+1, p[1]                    # nudge east
                intent = {"type":"intent","seq":int(time.time()*1000),"data":{"cmd":"move_to","x":tx,"y":ty}}
                await ws.send(json.dumps(intent))

asyncio.run(run())
