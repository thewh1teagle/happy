# Happy

Control [Happylight](https://play.google.com/store/apps/details?id=com.xiaoyu.hlight) compatible RGB LED strip using Python
<img style="width:150px;height:150px;" src="https://github.com/thewh1teagle/happy/assets/61390950/974fcdeb-9ba4-4372-9cac-1e0b96fbfed0"></img>

# Setup
```shell
git clone https://github.com/thewh1teagle/happy && cd happy
pip install -r requirements.txt
```

# Usage
```shell
python app.py
```

It will open simple web inerface
`scan`, `connect`, and then you will be able to control the `LED`


# build
```shell

cd ui && npm i && npm run build
cd app && pyinstaller --onefile --windowed --add-data "static;.\static" --icon=../assets/logo.png app.py
```