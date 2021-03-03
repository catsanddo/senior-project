import pygame as pg
pg.init()


class Entity:
    def __init__(self, x, y, sprite):
        self.x = x
        self.y = y
        self.sprite = pg.image.load(sprite)

    def draw(self, surface):
        surface.blit(self.sprite, (self.x, self.y))


def main():
    SCREEN = (512, 448)
    screen = pg.display.set_mode(SCREEN)

    s_buffer = pg.Surface((256, 224))

    player = Entity(0, 0, 'art/player.png')

    running = True
    while running:
        for e in pg.event.get():
            if e.type == pg.QUIT:
                running = False

        s_buffer.fill((0, 0, 0))
        player.draw(s_buffer)

        screen.blit(pg.transform.scale(s_buffer, SCREEN), (0, 0))
        pg.display.flip()


if __name__ == '__main__':
    main()
    pg.quit()
    quit()
