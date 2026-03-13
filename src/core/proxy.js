class PathTracker {
  #path = []

  constructor(target) {
    this.proxy = this.#createProxy(target, [])
  }

  #createProxy(target, currentPath) {
    const handler = {
      get: (obj, key, receiver) => {
        if (key === "getAccessPath") {
          return () => this.#path
        }

        if (key === "resetPath") {
          return () => {
            this.#path = []
          }
        }

        const newPath = [...currentPath, key]
        this.#path = newPath

        const value = Reflect.get(obj, key, receiver)

        if (typeof value === "object" && value !== null) {
          return this.#createProxy(value, newPath)
        }

        return value
      },
    }

    return new Proxy(target, handler)
  }

  /**
   * 获取最后一次访问的完整属性路径
   * @returns {string[]} 返回一个包含路径各部分的数组
   */
  getAccessPath() {
    return this.#path
  }

  /**
   * 重置已记录的访问路径
   */
  resetPath() {
    this.#path = []
  }
}

// --- 使用示例 ---

// 1. 定义一个复杂的原始对象
const data = {
  user: {
    posts: [{ id: 1, title: "First Post" }],
    profile: {
      contact: {
        email: "bob@example.com",
      },
      name: "Bob",
    },
  },
}

// 2. 创建 PathTracker 实例
const tracker = new PathTracker(data)

// 3. 访问 tracker.proxy 上的属性
// 每次访问都会更新内部的路径记录
console.log(tracker.proxy.user.profile.contact.email)

// 输出: "bob@example.com"

// 4. 调用实例方法获取路径
// 注意：要从 tracker 实例上调用方法，而不是从 proxy 上
console.log("访问路径:", tracker.getAccessPath())

// 输出: 访问路径: [ 'user', 'profile', 'contact', 'email' ]

// 5. 访问另一个属性
void tracker.proxy.user.posts[0].title

console.log("新的访问路径:", tracker.getAccessPath())

// 输出: 新的访问路径: [ 'user', 'posts', '0', 'title' ]

// 6. 重置路径
tracker.resetPath()
console.log("重置后的路径:", tracker.getAccessPath())

// 输出: 重置后的路径: []
