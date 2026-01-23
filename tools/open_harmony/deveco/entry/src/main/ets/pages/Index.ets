import {XComponentAttrs} from '../makepad/makepad';

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
      }
      .width('100%')
      .height('100%')
    }
    .height('100%')
    .width('100%')
  }
}
