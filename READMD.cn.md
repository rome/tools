![](https://github.com/facebookexperimental/rome/raw/master/assets/logo_with_text.png)

### 什么是roma

roma，中文名`罗马`，是Facebook的rn团队的产出，是一个试验性质的javascript工具链，集编译，linter，格式化，打包，测试等等于一体。目标是成为一个处理javascript源代码的全面性工具。

roma不是一个收集已有工具的整合体，所有的内部组件都是有业务逐渐提炼出来的，并且没有使用三方库。

roma虽然是试验性工具，但是目前正在积极地发展。目前对所有感兴趣的未来贡献者们敞开大门。因为roma还没有做好在生产环境使用的准备，所以使用roma的唯一方式是从源代码构建。

roma是有志向的工具链，它或许将替代很多已存在的javascript工具。当然，我们将来也会提供可以在其他工具中使用的组件。比如将roma作为插件集成在其他打包器中。

roma遵循[MIT协议](https://github.com/facebookexperimental/rome/blob/master/LICENSE)，并且基于[Contributor Covenant Code of Conduct](https://github.com/facebookexperimental/rome/blob/master/.github/CODE_OF_CONDUCT.md)管理。

### roma的诞生

[Sebastian McKenzie](https://twitter.com/sebmck)启动的roma项目，他同时也是babel和yarn的作者。

roma是Facebook的rn团队合力完成的项目。

roma的名字灵感来自于`条条大路通罗马`、`罗马非一日可就`和`在罗马的时候就要融入到罗马的生活中`。这些指代我们对roma的期望，希望整个项目中有广阔的适应范围和我们对约定(一致性)的渴望。然而。roma这个雄心勃勃的项目起初来自于办公室的一句玩笑话。 : )

roma有一个斯巴达头盔样式的logo，虽然可能与roma没有强相关性，但是它比[Galea](https://en.wikipedia.org/wiki/Galea_(helmet))要帅得多。


### 代码库

roma几乎完全使用typescript编写，几乎没有松散类型。

roma采用monorepo规范结构。

roma完全自托管，每次更新会使用之前的版本编译新的代码。

roma支持JSX和Flow、Typescript特有的注释代码。

[查看更多细节](https://github.com/facebookexperimental/rome/blob/master/.github/CONTRIBUTING.md)

### 当前集中点

当前我们主要致力于linting部分，参考[issue](https://github.com/facebookexperimental/rome/issues/20)


### 快速上手

如果想要在项目中集成roma，你所要做的只有一个配置文件：roma.json

```shell
$ mkdir hello-world
$ cd hello-world
$ echo '{}' >rome.json
```

该文件用来配置roma，并且也是划分是否受roma托管的重要标志。

更新细节请查看官网[](https://romejs.dev/docs/introduction/getting-started/)

### roma的哲学

下面的列表列出了roma信守的精神。这些内容是不全面的，其中一些是浅显易懂但是为了完成性依然将其列出。

#### 工程管理

- 设定明确的目标：提前制定工程的意图和期望，我们的工程不应该给我们惊喜！
- 公开性：有关roma的讨论和决定都将在一些公共场景，如github、roma dicord和推特。唯一例外的是对于代码的审核我们将严格保密。

#### 技术

- 无外部依赖：我们可以通过更机密的集成我们的内部工具和互通抽象概念，使我们更告诉的推动项目的进度和更加一致性的体验。利用定制型我们有更多的机会去获得更好的体验。
- 对错误提供修复提示：通过对常见使用方法的推断和过滤，我们要减少无关的、没有帮助的提示信息。
- 更精确到错误信息：我们没有通用的错误信息，因为这不能帮助使用者明白为什么会出错，也不能帮助开发者更好的通过错误信息定位问题所在。
- 简化API：对于可选配置和命令行参数，我们觉得不是必须的。他们并不能很好的组合使用。我们为什么不能简化这部分？
- 避免专用术语：这些专用术语除了装X几乎没有其他作用，我们致力于让新手和专家都能一目了然，例如，在编译出错的时候，使用'character'代替'token'！
- 对于命令和命令行参数避免缩写：没有必要时用令人迷惑且不得不查阅文档的缩写！
- 容错率高的术语：使用那些更容易明白的术语
- 兼容大部分终端：不要假设仅仅在terminal中使用，要兼容更多的通用环境。
- 使用强类型：不要使用松散类型，尽可能的验证输入。
- 终端上的输出更明确：设计终端输出的时候，不要仅仅依靠色彩来提高可读性，应合理运用格式、符号、间距来提高可读性。


### 社区

贡献和开发介绍在[这里](https://github.com/facebookexperimental/rome/blob/master/.github/CONTRIBUTING.md)

可以来这里[这里](https://discord.gg/9WxHa5d)一起讨论，但是要遵守我们的[准则](https://github.com/facebookexperimental/rome/blob/master/.github/CODE_OF_CONDUCT.md)哦！
