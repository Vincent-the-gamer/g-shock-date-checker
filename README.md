<div align="center">
    <img src=".github/logo.jpg" alt="logo" height="150"/>
</div>
<h1 align="center">G-SHOCK Date Checker</h1>
<p align="center">Check your G-SHOCK production date</p>

## G-SHOCK 8-digit Code
Casio G-SHOCK watches are using an 8-digit code to indicate which factory they are from, and the day they are built.

### Code Description:
![gshock](.github/g-shock.png)

The first 4 digits stand for the locations of factories.

~~~json
{
    "201A": "日本山形工厂 Yamagata, Japan",
    "201B": "日本甲府工厂 Kofu, Japan",
    "201C": "日本卡西欧工厂通用代码  Japan Universal Code",
    "201D": "日本山形卡西欧协力会社(合作伙伴）  Yamagata Casio Cooperation Association of Japan",
    "201E": "日本八王子工厂  Hachioji, Japan",
    "201F": "日本山形工厂  Yamagata, Japan",
    "202A": "泰国工厂  Thailand",
    "001A": "日本八王子工厂  Hachioji, Japan",
    "001C": "日本山形工厂  Yamagata, Japan",
    "002A": "泰国工厂  Thailand",
    "212A": "泰国工厂  Thailand",
    "220A": "中国广州工厂  Guangzhou, China",
    "003A": "中国工厂  China",
    "004A": "卡西欧中国中山工厂  Casio China Zhongshan Factory",
    "104A": "中国工厂  China",
    "204A": "中国工厂  China",
    "222A": "泰国工厂  Thailand"
}
~~~

The last 4 digits stand for the date of built.

264B: 
* 264: the 264th day of this year
* B: 
    * A-J: 1234567890
    * B can be 2012, 2022 or 2032 in the future, you need to understand when your watch released, then judge the exact date it was built.

#### The last digit:
~~~json
{
    "A": 1,
    "B": 2,
    "C": 3,
    "D": 4,
    "E": 5,
    "F": 6,
    "G": 7,
    "H": 8,
    "I": 9,
    "J": 0
}
~~~

### Target
This repo will provide:
* Executable files of server
* A web page

### Server

Call the only one API to get the G-Shock info.

#### Start Server
* Start with default port

~~~shell
# unix-like
./g-shock-server

# windows
g-shock-server.exe
~~~

Server will run at port 8081.

* Start with custom port

~~~shell
# unix-like
./g-shock-server 2333

# windows
g-shock-server.exe 2333
~~~

Server will run at port 2333.


#### Response of a successful request

**Example：**

Get `http://localhost:8081?code=201C264B`

~~~json
{
	"location": "日本卡西欧工厂通用Code(Japan Universal Code)",
	"productionDate": "2012.9.20, 2022.9.21"
}
~~~

#### Response of a failed request

**Example：**

Get `http://localhost:8081?code=asdasd`

~~~json
{
	"msg": "Your input is invalid! 你的输入不合法！"
}
~~~

#### Demo with ApiPost
![apipost](.github/apipost.png)


### Web Page
To be continued...