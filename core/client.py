import asyncio
from bleak import BleakScanner, BleakClient
from bleak.backends.device import BLEDevice
from .mode import Mode


class HappyClient():
    def __init__(self) -> None:
        self.UART_SERVICE_UUID = ""
        self.UART_RX_CHAR_UUID = ""
        self.UART_TX_CHAR_UUID = ""
        self.UART_SAFE_SIZE = 20

    def handle_disconnect(self, arg):
        pass

    async def connect(self, dev_or_addr: BLEDevice | str):
        self.client = BleakClient(dev_or_addr, disconnected_callback=self.handle_disconnect)
        try:
            await self.client.connect()
            for service in self.client.services:
                if service.description == "Generic Access Profile":
                    for char in service.characteristics:
                        print("Set UART_RX_CHAR_UUID with {}".format(char.uuid))
                        self.UART_RX_CHAR_UUID = char.uuid

                elif service.description == "Vendor specific":
                    for char in service.characteristics:
                        if (','.join(char.properties) == "write-without-response,write") and self.UART_TX_CHAR_UUID == "":
                            self.UART_TX_CHAR_UUID = char.uuid
                            
        except asyncio.CancelledError as ex:
            print('Cant connect device. try with HappyLighting app https://play.google.com/store/apps/details?id=com.xiaoyu.hlight')

    async def disconnect(self):
        try:
            # await self.client.disconnect()
            pass # TODO: disconnect, currently it hangs...
        except asyncio.CancelledError as ex:
            pass

    async def set_rgb(self, r, g, b):
            args = [86, r, g, b, (int(10 * 255 / 100) & 0xFF), 256-16, 256-86]
            values = bytearray(args)
            try:
                await self.client.write_gatt_char(self.UART_TX_CHAR_UUID, values, False)
            except Exception as inst:
                print(inst)

    async def set_power(self, state: bool):
            values = bytearray([204, 35, 51] if state else [204, 36, 51])
            try:
                await self.client.write_gatt_char(self.UART_TX_CHAR_UUID, values, False)
            except Exception as inst:
                print(inst)

    async def set_mode(self, mode: Mode, speed = 0):
            #new byte[] { 256 - 69, mode, (byte)(speed & 0xFF), 68 };
            lista = [256 - 69, mode, (speed & 0xFF), 68]
            values = bytearray(lista)
            try:
                await self.client.write_gatt_char(self.UART_TX_CHAR_UUID, values, False)
            except Exception as inst:
                print(inst)

    async def set_mic(self, status):
            if status:
                var_1 = 256 - 16
                var_2 = 50
            else:
                var_1 = 15
                var_2 = 30
            args = [1, var_1, var_2, 0 , 0, 24]
            values = bytearray(args)
            try:
                await self.client.write_gatt_char(self.UART_TX_CHAR_UUID, values, False)
            except Exception as inst:
                print(inst)

    async def scan(self, timeout=3):
        return await BleakScanner.discover(timeout=timeout)
         

async def main():
    client = HappyClient()
    devices = await client.scan()
    await client.connect(devices[0])
    await client.set_mode(Mode.PULSATING_GREEN.value)
    await client.set_power(True)
    await client.disconnect()

if __name__ == '__main__':
    asyncio.run(main())