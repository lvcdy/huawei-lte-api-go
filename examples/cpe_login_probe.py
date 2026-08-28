#!/usr/bin/env python3
"""对照实验：用官方 Python 库连接 H168-383，验证登录。

只做登录，不调用任何业务 API。输出每一步的关键信息：
- 首页 token 数
- state-login 的 password_type / rsapadingtype / State
- 登录请求体（编码后密码截断显示）
- 登录结果
"""
import base64
import hashlib
import sys

from huawei_lte_api.Session import Session
from huawei_lte_api.api.User import User

URL = sys.argv[1] if len(sys.argv) > 1 else "http://192.168.8.1/"
USERNAME = sys.argv[2] if len(sys.argv) > 2 else "admin"
PASSWORD = sys.argv[3] if len(sys.argv) > 3 else "password"


def main() -> None:
    s = Session(URL)
    token_count = len(s.request_verification_tokens)
    print(f"request_verification_tokens: {token_count}")
    print(f"  token[0] = {s.request_verification_tokens[0] if token_count else '(none)'}")
    if token_count > 1:
        print(f"  token[1] = {s.request_verification_tokens[1]}")

    user = User(s)
    try:
        state = user.state_login()
    except Exception as e:  # noqa: BLE001
        print(f"state_login FAILED: {type(e).__name__}: {e}")
        return

    print(f"state-login: password_type={state.get('password_type')} "
          f"rsapadingtype={state.get('rsapadingtype')} State={state.get('State')}")

    # 手动计算并展示编码后的密码（对照 Go 实现）
    pw_type = int(state.get("password_type", 0))
    if pw_type == 4:
        concentrated = b"".join([
            USERNAME.encode("UTF-8"),
            base64.b64encode(hashlib.sha256(PASSWORD.encode("UTF-8")).hexdigest().encode("ascii")),
            s.request_verification_tokens[0].encode("UTF-8"),
        ])
        enc = base64.b64encode(hashlib.sha256(concentrated).hexdigest().encode("ascii"))
    else:
        enc = base64.b64encode(PASSWORD.encode("UTF-8"))
    print(f"encoded password (type {pw_type}): {enc[:40]}...")

    login_ok = user.login(USERNAME, PASSWORD, force_new_login=True)
    print(f"login result: {'OK' if login_ok else 'FAIL'}")

    # 若登录成功，再验证一个业务 API
    if login_ok:
        try:
            state_after = user.state_login()
            print(f"state after login: State={state_after.get('State')}")
        except Exception as e:  # noqa: BLE001
            print(f"state_login after login FAILED: {type(e).__name__}: {e}")
    s.close()


if __name__ == "__main__":
    main()