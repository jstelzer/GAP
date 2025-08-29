#!/usr/bin/env python
import asyncio, json, websockets, time

async def run():
    print("Connecting to WebSocket server...")
    async with websockets.connect("ws://127.0.0.1:7777") as ws:
        print("Connected! Sending hello message...")
        hello_msg = {"type":"hello","version":"0.2.0","agent":"test"}
        print(f"Sending: {hello_msg}")
        await ws.send(json.dumps(hello_msg))
        
        print("Listening for messages...")
        async for msg in ws:
            m = json.loads(msg)
            print(f"Received message type: {m.get('type')}")
            
            if m.get("type")=="state":
                p = m["data"]["player"]["pos"]
                print(f"Player position: {p}")
                tx, ty = p[0]+1, p[1]                    # nudge east
                print(f"Moving to: ({tx}, {ty})")
                intent = {"type":"intent","seq":int(time.time()*1000),"data":{"cmd":"move_to","x":tx,"y":ty}}
                print(f"Sending intent: {intent}")
                await ws.send(json.dumps(intent))
            elif m.get("type")=="hello":
                print(f"Server hello: {m}")
            elif m.get("type")=="ack":
                print(f"Received acknowledgment: {m}")
            else:
                print(f"Unknown message: {m}")

asyncio.run(run())
