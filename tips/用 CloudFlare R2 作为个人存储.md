tags:: tips, daily, 日常, storage


自己不喜欢搞太多设备在家里,
CloudFlare R2 有 10G 的免费存储额度并且下行流量不收费,
可以作为个人日常大文件存储服务囤电影什么的:

https://www.cloudflare.com/lp/pg-r2-comparison/

| Free Tier                 | Cloudflare R2        |  Amazon S3              |
| :--                       | :--                  | :--                     |
| Storage                   | 10 GB per month      | 5 GB per month          |
| Class A Operations (POST) | 1 Million per month  | 2,000 per month         |
| Class B Operations (GET)  | 10 Million per month | 20,000 per month        |
| Data Transfer             | Unlimited            | 100 GB                  |

| Pricing After Free Tier   | Cloudflare R2        |  Amazon S3                |
| :--                       | :--                  | :--                       |
| Storage / Month           | `$0.015` per GB      | Starts at `$0.023` per GB |
| Class A Operations (POST) | `$0.0045` per 1,000  | `$0.005` per 1,000        |
| Class B Operations (GET)  | `$0.00036` per 1,000 | `$0.0004` per 1,000       |
| Egress Fees               | FREE                 | Starts at `$0.09` per GB  |


# 开通 R2 服务

打开url,  填写信用卡, 搞定:

https://dash.cloudflare.com/?to=/:account/billing/checkout&add=r2_basic


## 创建一个bucket

例如movie-xp, 然后就可以用这个bucket囤电影了.


# 测试使用

## aws CLI 工具

因为 R2 兼容 aws s3, 可以使用命令行工具 `aws` 来管理 R2 的存储.

### 配置 aws 

- 在 R2 上生成一堆access key和secret key, 用于在CLI中允许aws访问R2服务:
  https://dash.cloudflare.com/eb2e7f3ee6f76892d19d15b6cb3c5bab/r2/api-tokens

  点击生成, 记下

- 配置 aws 使用上面生成的access key 和secret key:

    ```
    $ aws configure
    AWS Access Key ID [None]: <access_key_id>
    AWS Secret Access Key [None]: <access_key_secret>
    Default region name [None]: auto
    Default output format [None]: json
    ```

    这个命令会在 home 目录下生成 2个配置文件:
    ```
    /Users/drdrxp/
    ▾ .aws/
        config
        credentials
    ```

- 配置 aws 去访问 R2 服务:

    打开`HOME/.aws/config`, 添加一个指向 R2的 `endpoint_url` 到 default section:

    ```
    [default]
    region = auto
    output = json

    endpoint_url = https://account_id.r2.cloudflarestorage.com
    ```
    其中`account_id`替换成自己的, 例如:

    ```
    endpoint_url = https://eb............................ab.r2.cloudflarestorage.com
    ```

    `account_id` 可以在 R2 主页找到, 进CloudFlare,  R2, Overview, 在页面右上角:

    `https://dash.cloudflare.com/eb............................ab/r2/overview`

最后确认下配置OK, 命令行下运行, 列出bucket:
```
aws s3 ls
2023-09-17 11:07:56 movie-xp
```

### Optional: 更多配置:

配置文件里可以配置除default 之外更多的 `profile`, 例如:

https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-format

```
[default]
sso_session = my-sso
sso_account_id = 111122223333
sso_role_name = readOnly
region = us-west-2
output = text

[profile user1]
sso_session = my-sso
sso_account_id = 444455556666
sso_role_name = readOnly
region = us-east-1
output = json

[sso-session my-sso]
sso_region = us-east-1
sso_start_url = https://my-sso-portal.awsapps.com/start
sso_registration_scopes = sso:account:access
```

也支持命令行, 环境变量等配置方式:
https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html#cli-configure-quickstart-config


### 上传下载

上传单个文件:

```
aws s3 cp ./Gran-Torino-2008-老爷车.rmvb s3://movie-xp/
```

下载单个文件:

```
aws s3 cp s3://movie-xp/Gran-Torino-2008-老爷车.rmvb ./
```

本地上传速度大约 4 MB/s; 下载大约 10 MB/s;

### low-level 命令

`aws s3` 提供日常使用方便的命令,  `aws s3api` 提供底层命令. 


## GUI: Commander One

试了下 [Commander One][] 还不错(cyber duck中配置一个自定义`endpoint_url`会把服务强制为webdav,
导致无法使用R2(只能使用aws原始域名, 不知哪里配置错了));


然后从菜单 Commander One - Connection Manager中创建一个到R2的链接:
需要配置3个选项: access key, secret key 和 `endpoint_url`.

### 缺陷

Commander One 中无法在R2中创建目录, 这是因为s3 协议中没有目录的概念, Commander
One也没有做任何模拟目录的支持;

无法直接创建目录有些不方便, 目录为空时就自动删除了. 
可以通过指定带目录的key上传的方式来实现同时上传文件和创建目录.



[Commander One]: https://ftp-mac.com/

