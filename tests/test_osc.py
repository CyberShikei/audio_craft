#!/bin/python
from pythonosc.udp_client import SimpleUDPClient

client = SimpleUDPClient("127.0.0.1", 9000)
client.send_message("/test", 42)  # Send a test message
