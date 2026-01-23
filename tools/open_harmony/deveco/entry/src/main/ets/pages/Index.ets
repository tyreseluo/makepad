import makepad from 'libmakepad.so';
import {ArkGlue, MakepadXComponentInterface, XComponentAttrs} from '../makepad/makepad';

@Entry
@Component
struct Index {
  xComponentAttrs: XComponentAttrs = {
    id: 'Entry',
    type: XComponentType.SURFACE,
    libraryname: 'makepad',
  }

  build() {
    Row() {
      Column() {
        XComponent(this.xComponentAttrs)
          .focusable(true)
          .width('100%')
          .height('100%')
          .onLoad((xComponentContext) => {
            makepad.registerXComponent(xComponentContext as object);
            ArkGlue.getInstance().xComponentContext = makepad as MakepadXComponentInterface;
          })
      }
      .width('100%')
      .height('100%')
    }
    .height('100%')
    .width('100%')
  }
}
