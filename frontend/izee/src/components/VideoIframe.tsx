interface YoutubeVideoProps {
  videoId: string;
}
export const YoutubeVideo = ({ videoId }: YoutubeVideoProps) => {
  return (
    <iframe
      width="400"
      height="225"
      src={`https://www.youtube.com/embed/${videoId}`}
      title={videoId}
      allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    />
  );
};
