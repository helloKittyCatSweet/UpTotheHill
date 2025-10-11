import os
import re
import requests
from bs4 import BeautifulSoup
from PIL import Image, ImageEnhance, ImageFilter
from io import BytesIO
import easyocr
import numpy as np

# 初始化 EasyOCR
reader = easyocr.Reader(['en'], gpu=False)  # 仅英文识别即可

album_url = "https://www.douban.com/photos/album/145972492/?m_start=72"
save_dir = "douban_english_ocr"
os.makedirs(save_dir, exist_ok=True)

headers = {
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
                  "AppleWebKit/537.36 (KHTML, like Gecko) "
                  "Chrome/120.0.0.0 Safari/537.36"
}

response = requests.get(album_url, headers=headers)
response.raise_for_status()
soup = BeautifulSoup(response.text, "html.parser")

img_tags = soup.find_all("img", {"src": re.compile(r"^https://img\d\.doubanio\.com/")})
img_urls = [img["src"].replace("/m/", "/l/") for img in img_tags]

print(f"共找到 {len(img_urls)} 张图片")

def enhance_image(image):
    """图像增强：提高对比度、锐化、去噪"""
    image = image.convert("RGB")
    enhancer = ImageEnhance.Contrast(image)
    image = enhancer.enhance(1.8)
    image = image.filter(ImageFilter.MedianFilter(size=3))
    return image

for idx, url in enumerate(img_urls, start=1):
    try:
        img_data = requests.get(url, headers=headers).content
        image = Image.open(BytesIO(img_data))
        image = enhance_image(image)

        # 使用 EasyOCR 识别
        result = reader.readtext(np.array(image), detail=0)
        text = " ".join(result).strip()
        text = re.sub(r"[\\/:*?\"<>|]", "_", text)
        text = text[:40] if text else f"photo_{idx}"

        filename = os.path.join(save_dir, f"{text}.jpg")
        image.save(filename)
        print(f"✅ 已保存: {filename}")
    except Exception as e:
        print(f"❌ 处理失败 {url}: {e}")
