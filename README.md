# image_thumbnail_generator
creates thumbnails of images in a directory using rayon parallel iteration

!Note: there is a bug in this code. Right now it only works if there is a thumbnails/images directory in the root directory
If there is such a directory then the thumbnails are generated successfully

TODO: 
---

- [ ] Fix save_path variable in the make_thumbnail function so files can be saved in the folder provided. To fix that Path may need be sliced
