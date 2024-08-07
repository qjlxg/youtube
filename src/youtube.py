import datetime

import requests
import re
import urllib.parse
from google_drive_downloader import GoogleDriveDownloader as gdd
from pytz import timezone

headers = {
    "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
    "accept-language": "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7",
    "cache-control": "max-age=0",
    "cookie": "YSC=FZAqgHLfdCM; CONSENT=YES+cb.20210420-15-p1.zh-CN+FX+732; GPS=1; VISITOR_INFO1_LIVE=v2aixc6NJXI; PREF=tz=Asia.Shanghai",
    "referer": "https://www.youtube.com/",
    "sec-ch-ua": '" Not A;Brand";v="99", "Chromium";v="90", "Google Chrome";v="90"',
    "sec-ch-ua-mobile": "?0",
    "sec-fetch-dest": "document",
    "sec-fetch-mode": "navigate",
    "sec-fetch-site": "same-origin",
    "sec-fetch-user": "?1",
    "service-worker-navigation-preload": "true",
    "upgrade-insecure-requests": "1",
    "user-agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.85 Safari/537.36",
    "x-client-data": "CI+2yQEIpLbJAQjEtskBCKmdygEIlqzKAQiIucoBCPjHygEI1uvKAQjjnMsBCKmdywEI6J3LAQigoMsBGOGaywE=",
}


# 爬取到视频主页，获取视频的播放ID
def getIDs(url, headers):
    response = requests.get(url, headers=headers, timeout=None, verify=False)
    response.encoding = "utf-8"
    print(response.status_code)
    print(len(response.text))
    ids = re.findall(r"(/watch\?v=.+?)\042", response.text)
    return ids


# 获取订阅链接页的详情
def getSubLink(url, headers):
    r = requests.get(url, headers=headers, timeout=None, verify=False)
    r.encoding = "utf-8"
    links = re.findall(
        r"(skill-note\.blogspot\.com.*\.html)\042,",
        r.text,
    )
    print(links)
    links = [urllib.parse.unquote(i) for i in links]
    if len(links) > 0:
        return "https://" + links[0]
    else:
        return None


# 获取视频播放页的详情
def getContent(url, headers):
    r = requests.get(url, headers=headers, timeout=None, verify=False)
    r.encoding = "utf-8"
    # print(r.text)
    file_ids = re.findall(r"https://drive\.google\.com/file/d/([\w-]*)", r.text)
    return file_ids
    # print(file_ids[0])


if __name__ == "__main__":
    try:
        url = "https://www.youtube.com/channel/UCOQ5AdvDNOfyEAJY5SDXVZg/videos"
        # print('https://www.youtube.com'+getIDs(url,headers)[0][:-2])
        ids = getIDs(url, headers)
        print(ids)
        # 获取最新一期的播放地址
        video_url = "https://www.youtube.com" + ids[0]
        print(video_url)
        sub_url = getSubLink(video_url, headers)
        print(sub_url)
        if sub_url:
            googleDrive_ids = getContent(sub_url, headers)
            print(googleDrive_ids)
            names = ["clash-meta.yaml", "clash-old.yaml", "ss", "v2ray","hy2"]
            for gid, name in zip(googleDrive_ids, names):
                # gdd.download_file_from_google_drive(
                #     file_id=googleDrive_ids[0],
                #     dest_path="./YT/{}.yaml".format(
                #         datetime.datetime.now(timezone("Asia/Shanghai")).strftime(
                #             "%Y-%m-%d %H:%M"
                #         )
                #     ),
                #     showsize=True,
                #     overwrite=True,
                # )
                gdd.download_file_from_google_drive(
                    file_id=gid,
                    dest_path="./sub/youtube/" + name,
                    showsize=True,
                    overwrite=True,
                )
                # requests.get("https://api.day.app/3TKmw24emfnWtLN6xyDaW9/YouTube节点爬取成功{}".format(datetime.datetime.now(timezone('Asia/Shanghai')).strftime('%Y-%m-%d %H:%M')))
            # else:
            #     print("YouTube节点爬取失败")
            # requests.get("https://api.day.app/3TKmw24emfnWtLN6xyDaW9/YouTube节点爬取失败{}".format(
            #     datetime.datetime.now(timezone('Asia/Shanghai')).strftime('%Y-%m-%d %H:%M')))
            # downYaml(url,headers)
            # print("down")
    except Exception as r:
        print(r)
